import subprocess
import argparse
import re

def distance_u32(start, end):
    assert abs(end - start) < 2 ** 31
    diff = end - start
    if diff < 0:
        return 2 ** 32 + diff
    else:
        return diff

parser = argparse.ArgumentParser()
parser.add_argument("binary_path", help="Binary path")
parser.add_argument("-o", "--output", help="Output path", required=True)
args = parser.parse_args()

with open(args.binary_path, "rb") as f:
    binary = bytearray(f.read())

readelf = subprocess.check_output(["readelf", "-S", args.binary_path]).decode("utf-8").splitlines()
got = None
for line in readelf:
    if ".got" in line:
        got = int(line[line.index(".got"):].split()[2], 16)
        break

if got is None:
    with open(args.output, "wb") as f:
        f.write(binary)
    exit(0)

objdump = subprocess.check_output(["objdump", "-Fd", args.binary_path]).decode("utf-8").splitlines()
sym_regex = re.compile(r"([0-9a-f]{16}) <.+> \(File Offset: 0x([0-9a-f]+)\)")
got_regex = re.compile(r" *([0-9a-f]+):\t([0-9a-f ]+).+# ([0-9a-f]+) <.+> \(File Offset: 0x([0-9a-f]+)\)")
sym_addr = 0
sym_offset = 0
for line in objdump:
    match = sym_regex.match(line)
    if match:
        sym_addr = int(match.group(1), 16)
        sym_offset = int(match.group(2), 16)
    match = got_regex.match(line)
    if match and int(match.group(3), 16) >= got:
        addr_offset = int(match.group(4), 16)
        addr = int.from_bytes(binary[addr_offset:addr_offset+8], 'little')
        virt = int(match.group(1), 16)
        asm_offset = virt - sym_addr + sym_offset
        asm = match.group(2).split()
        if asm[0] == "ff":
            if asm[1] == "15":
                binary[asm_offset:asm_offset+6] = b"\xe8" + distance_u32(virt + 5, addr).to_bytes(4, "little") + b"\x90"
            elif asm[1] == "25":
                binary[asm_offset:asm_offset+6] = b"\xe9" + distance_u32(virt + 5, addr).to_bytes(4, "little") + b"\x90"
            else:
                print(f"Unknown instruction: {line}")
        else:
            print(f"Unknown instruction: {line}")

with open(args.output, "wb") as f:
    f.write(binary)

