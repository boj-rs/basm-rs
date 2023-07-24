import array
import base64
import lzma
import re
import subprocess
import sys

try:
    solution_src_path, elf_path, stub_path, lang_name, template_path = sys.argv[1:]
except ValueError:
    print(f"Usage: {sys.argv[0]} solution_src_path elf_path stub_path lang_name template_path", file=sys.stderr)
    sys.exit(1)
if lang_name not in ["C", "Rust"]:
    print(f"Unsupported language {lang_name}", file=sys.stderr)
    sys.exit(1)

binary_path = elf_path + ".bin"
compressed_binary_path = binary_path + ".lzma"
elf2bin = subprocess.check_output(["python3", "scripts/static-pie-elf2bin.py", elf_path, binary_path]).decode("utf-8").splitlines()
entrypoint_offset_str = None
for line in elf2bin:
    y = line.strip()
    if len(y) == 0:
        continue
    entrypoint_offset_str = y
    break
assert entrypoint_offset_str is not None

# Please refer to the following link for the lzma file format:
#   https://svn.python.org/projects/external/xz-5.0.3/doc/lzma-file-format.txt
with open(binary_path, "rb") as f:
    memory_bin = f.read()
compressed_memory_bin = lzma.compress(memory_bin, format=lzma.FORMAT_ALONE, preset=lzma.PRESET_EXTREME)
compressed_memory_bin = bytearray(compressed_memory_bin)
compressed_memory_bin[5:13] = len(memory_bin).to_bytes(8, byteorder='little')   # fill the "Uncompressed Size" field
compressed_memory_bin = bytes(compressed_memory_bin)
with open(compressed_binary_path, "wb") as f:
    f.write(compressed_memory_bin)

# solution_src
with open(solution_src_path, encoding='utf8') as f:
    sol = f.readlines()

sol = [line.replace("\ufeff", "") for line in sol]
sol = ["//" + line.rstrip() + "\n" for line in sol]
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
stub = base64.b85encode(stub, pad=False).decode('ascii') + ']'
if lang_name == "C":
    stub = stub.replace("?", "\?")
stub = '"' + stub + '"'

# template
with open(template_path, encoding='utf8') as f:
    template = f.read()

# putting it all together
# reference: https://stackoverflow.com/a/15448887
def multiple_replace(string, rep_dict):
    pattern = re.compile("|".join([re.escape(k) for k in sorted(rep_dict,key=len,reverse=True)]), flags=re.DOTALL)
    return pattern.sub(lambda x: rep_dict[x.group(0)], string)

out = multiple_replace(template, {
    "$$$$solution_src$$$$": sol,
    "$$$$stub_base85$$$$": stub,
    "$$$$binary_base85$$$$": r,
    "$$$$binary_base85_len$$$$": str(len(code_b85)),
    "$$$$entrypoint_offset$$$$": entrypoint_offset_str,
})
print(out)