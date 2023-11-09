import array
import base64
import codecs
import json
import lzma
import os
import re
import subprocess
import sys

try:
    solution_src_path, target_name, elf_path, stub_path, lang_name, template_path = sys.argv[1:]
except ValueError:
    print(f"Usage: {sys.argv[0]} solution_src_path target_name elf_path stub_path lang_name template_path", file=sys.stderr)
    sys.exit(1)
if lang_name not in ["C", "Rust"]:
    print(f"Unsupported language {lang_name}", file=sys.stderr)
    sys.exit(1)

if target_name == "x86_64-pc-windows-msvc":
    binary_path = elf_path + ".bin"
    compressed_binary_path = binary_path + ".lzma"
    elf2bin = subprocess.check_output([sys.executable, "scripts/static-pie-pe2bin.py", elf_path, binary_path]).decode("utf-8")
else:
    binary_path = elf_path + ".bin"
    compressed_binary_path = binary_path + ".lzma"
    elf2bin = subprocess.check_output([sys.executable, "scripts/static-pie-elf2bin.py", elf_path, binary_path]).decode("utf-8")
loader_fdict = json.loads(elf2bin)
assert 'leading_unused_bytes' in loader_fdict
assert 'entrypoint_offset' in loader_fdict
assert 'pe_image_base' in loader_fdict
assert 'pe_off_reloc' in loader_fdict
assert 'pe_size_reloc' in loader_fdict

# Please refer to the following link for the lzma file format:
#   https://svn.python.org/projects/external/xz-5.0.3/doc/lzma-file-format.txt
with open(binary_path, "rb") as f:
    memory_bin = f.read()
    # Embed these information into the LZMA file to reduce the generated code length
    x = loader_fdict['pe_image_base'].to_bytes(8, byteorder='little') + \
        loader_fdict['pe_off_reloc'].to_bytes(8, byteorder='little') + \
        loader_fdict['pe_size_reloc'].to_bytes(8, byteorder='little') + \
        loader_fdict['entrypoint_offset'].to_bytes(8, byteorder='little')
    memory_bin += x
lzma_filter = {'id': lzma.FILTER_LZMA1, 'preset': lzma.PRESET_EXTREME, 'lp': 0, 'lc': 0, 'pb': 2, 'dict_size': 1 << 22}
compressed_memory_bin = lzma.compress(memory_bin, format=lzma.FORMAT_RAW, filters=[lzma_filter])
lzma_header_properties = ((lzma_filter['pb'] * 5 + lzma_filter['lp']) * 9 + lzma_filter['lc']).to_bytes(1, byteorder='little')
lzma_header_dictionary_size = lzma_filter['dict_size'].to_bytes(4, byteorder='little')
lzma_header_uncompressed_size = len(memory_bin).to_bytes(8, byteorder='little')
compressed_memory_bin = lzma_header_properties + lzma_header_dictionary_size + lzma_header_uncompressed_size + bytes(compressed_memory_bin)
with open(compressed_binary_path, "wb") as f:
    f.write(compressed_memory_bin)

# solution_src
with open(solution_src_path, encoding='utf8') as f:
    sol = f.readlines()

sol = [line.replace("\ufeff", "") for line in sol]
sol = [("" if lang_name == "Rust" else "//") + line.rstrip() + "\n" for line in sol]
if len(sol) > 0:
    sol[-1] = sol[-1].rstrip()
sol = "".join(sol)

# binary
with open(compressed_binary_path, "rb") as f:
    code = f.read()

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
with open(template_path, encoding='utf8') as f:
    template = f.read()
template = template.replace("\ufeff", "")

# putting it all together
# reference: https://stackoverflow.com/a/15448887
def multiple_replace(string, rep_dict):
    pattern = re.compile("|".join([re.escape(k) for k in sorted(rep_dict,key=len,reverse=True)]), flags=re.DOTALL)
    return pattern.sub(lambda x: rep_dict[x.group(0)], string)

out = multiple_replace(template, {
    "$$$$solution_src$$$$": sol,
    "$$$$stub_raw$$$$": stub_raw,
    "$$$$stub_base85$$$$": stub_b85,
    "$$$$stub_len$$$$": str(len(stub)),
    "$$$$stub_base85_len$$$$": str(stub_b85_len),
    "$$$$binary_base85$$$$": r,
    "$$$$binary_base85_len$$$$": str(len(code_b85)),
    "$$$$min_len_4096$$$$": str(min(len(code_b85)+1, 4096)),
    "$$$$leading_unused_bytes$$$$": str(loader_fdict['leading_unused_bytes']),
    "$$$$entrypoint_offset$$$$": str(loader_fdict['entrypoint_offset']),
    "$$$$pe_image_base$$$$": str(loader_fdict['pe_image_base']),
    "$$$$pe_off_reloc$$$$": str(loader_fdict['pe_off_reloc']),
    "$$$$pe_size_reloc$$$$": str(loader_fdict['pe_size_reloc']),
})
print(out)