import array
import base64
import base91
import bindgen.parse, bindgen.emit
import codecs
import json
import locator
import lzma
import os
import srcpack
import subprocess
import sys
import utils

try:
    solution_src_path, target_name, elf_path, stub_path, lang_name, template_path = sys.argv[1:]
    stub_path = locator.template_path(stub_path)
    template_path = locator.template_path(template_path)
except ValueError:
    print(f"Usage: {sys.argv[0]} solution_src_path target_name elf_path stub_path lang_name template_path", file=sys.stderr)
    sys.exit(1)
if lang_name not in ["C", "Rust"]:
    print(f"Unsupported language {lang_name}", file=sys.stderr)
    sys.exit(1)

if target_name in ["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"]:
    binary_path = elf_path + ".bin"
    compressed_binary_path = binary_path + ".lzma"
    elf2bin = subprocess.check_output([sys.executable, "scripts/static-pie-pe2bin.py", elf_path, binary_path]).decode("utf-8")
else:
    binary_path = elf_path + ".bin"
    compressed_binary_path = binary_path + ".lzma"
    elf2bin = subprocess.check_output([sys.executable, "scripts/static-pie-elf2bin.py", elf_path, binary_path]).decode("utf-8")
loader_fdict = json.loads(elf2bin)
assert 'entrypoint_offset' in loader_fdict

# Please refer to the following link for the lzma file format:
#   https://svn.python.org/projects/external/xz-5.0.3/doc/lzma-file-format.txt
# However, we use a different format:
#   [ 0,  1) = (1 << pb) - 1
#   [ 1,  2) = (1 << lp) - 1
#   [ 2,  3) = lc
#   [ 3,  4) = lp + lc + 8
#   [ 4,  8) = Uncompressed size
#   [ 8, ..) = Compressed data without the leading byte
with open(binary_path, "rb") as f:
    memory_bin = f.read()
    # Embed these information into the LZMA file to reduce the generated code length
    x = loader_fdict['entrypoint_offset'].to_bytes(8, byteorder='little')
    memory_bin += x
lzma_filter = {'id': lzma.FILTER_LZMA1, 'preset': lzma.PRESET_EXTREME, 'lp': 0, 'lc': 0, 'pb': 0, 'dict_size': 1 << 22, 'depth': 200}
compressed_memory_bin = bytearray(lzma.compress(memory_bin, format=lzma.FORMAT_RAW, filters=[lzma_filter]))
while len(compressed_memory_bin) < 4:
    compressed_memory_bin += b'\x00'                # append zeros for byte order swap (this won't happen in almost all cases, though)
compressed_memory_bin = compressed_memory_bin[1:]   # strip the (redundant) leading zero byte of the LZMA stream
compressed_memory_bin[:4] = reversed(compressed_memory_bin[:4]) # perform byte order swap in advance

pb, lp, lc = lzma_filter['pb'], lzma_filter['lp'], lzma_filter['lc']
lzma_header_properties = ((((1 << pb) - 1) + ((1 << lp) - 1) << 8) + (lc << 16) + ((lp + lc + 8) << 24)).to_bytes(4, byteorder='little')
lzma_header_uncompressed_size = len(memory_bin).to_bytes(4, byteorder='little')
compressed_memory_bin = lzma_header_properties + lzma_header_uncompressed_size + bytes(compressed_memory_bin)
with open(compressed_binary_path, "wb") as f:
    f.write(compressed_memory_bin)

# solution_src
sol = srcpack.read_assemble("basm/", lang_name)

# binary (raw)
# Since we append a little-endian 8-byte nonnegative integer, we can practically ensure that the last byte is zero.
code_raw = memory_bin[:-8]
code_raw += (len(code_raw) + 8 - loader_fdict['entrypoint_offset']).to_bytes(8, byteorder='little')
code_raw_b91 = base91.encode(code_raw, use_rle=True).decode('ascii')
code_raw_b91_len = len(code_raw_b91)
code_raw_b91 = '"' + code_raw_b91 + '"'
if lang_name == "C":
    # Escape '\' and '?'
    code_raw_b91 = code_raw_b91.replace('\\', '\\\\')
    code_raw_b91 = code_raw_b91.replace('?', '\\?')

# binary
with open(compressed_binary_path, "rb") as f:
    code = f.read()

code_b91 = base91.encode(code).decode('ascii')
code_b91_len = len(code_b91)
code_b91 = '"' + code_b91 + '"'

code = bytearray(code)
while len(code) % 4 != 0:
    code.append(0)
code_b85 = base64.b85encode(code, pad=False).decode('ascii') + ']'

if lang_name == "C":
    L = 4095
    s = []
    for i in range(0, len(code_b85), L):
        x = code_b85[i:min(i+L,len(code_b85))]
        x = x.replace("?", "\?")
        x = '"' + x + '",\n'
        s.append(x)
    r = "{\n" + "".join(s) + "}"
else:
    r = '"' + code_b85 + '"'

# stub
with open(stub_path, "rb") as f:
    stub = f.read()
if lang_name == "Rust" and "x86_64" in target_name:
    with open(stub_path.replace("stub-amd64", "prestub-amd64-2"), "rb") as f:
        prestub2 = f.read()
    stub = prestub2 + stub

stub_b91 = base91.encode(stub).decode('ascii')
stub_b91_len = len(stub_b91)
stub_b91 = '"' + stub_b91 + '"'

stub = bytearray(stub)
while len(stub) % 4 != 0:
    stub.append(0)
stub_raw = '"' + "".join("\\x{:02x}".format(x) for x in stub) + '"'
stub_b85 = base64.b85encode(stub, pad=False).decode('ascii') + ']'
stub_b85_len = len(stub_b85)
if lang_name == "C":
    stub_b85 = stub_b85.replace("?", "\?")
stub_b85 = '"' + stub_b85 + '"'

# template
template_candidates = [template_path]
if lang_name in ["C", "Rust"] and "x86_64" in target_name and "short" in template_path and len(code_raw) <= 4096 - 256:
    template_candidates.append(template_path.replace("short", "shorter"))

# exports
exports_dict = loader_fdict.get("exports", dict())
exports_imports_list = []
for e_name, e_offset in exports_dict.items():
    sig = bindgen.parse.Signature(e_name)
    exports_imports_list.append((sig, e_offset))
exports_cpp = bindgen.emit.emit_all(exports_imports_list)

out = None
for each_template_path in template_candidates:
    with open(each_template_path, encoding='utf8') as f:
        template = f.read()
    template = template.replace("\ufeff", "")

    out_candidate = utils.multiple_replace(template, {
        "$$$$solution_src$$$$": sol,
        "$$$$stub_raw$$$$": stub_raw,
        "$$$$stub_base85$$$$": stub_b85,
        "$$$$stub_len$$$$": str(len(stub)),
        "$$$$stub_base85_len$$$$": str(stub_b85_len),
        "$$$$stub_base91$$$$": stub_b91,
        "$$$$stub_base91_len$$$$": str(stub_b91_len),
        "$$$$binary_base85$$$$": r,
        "$$$$binary_base85_len$$$$": str(len(code_b85)),
        "$$$$binary_base91$$$$": code_b91,
        "$$$$binary_base91_len$$$$": str(code_b91_len),
        "$$$$binary_raw_base91$$$$": code_raw_b91,
        "$$$$binary_raw_base91_len$$$$": str(code_raw_b91_len),
        "$$$$min_len_4096$$$$": str(min(len(code_b85)+1, 4096)),
        "$$$$entrypoint_offset$$$$": str(loader_fdict['entrypoint_offset']),
        "$$$$exports_cpp$$$$": exports_cpp
    })
    if out is None or len(out_candidate) < len(out):
        out = out_candidate
print(out)