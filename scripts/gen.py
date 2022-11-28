import array
import sys

try:
    binary_path, template_path = sys.argv[1:]
except ValueError:
    print(f"Usage: {sys.argv[0]} binary_path template_path", file=sys.stderr)
    sys.exit(1)

with open(binary_path, "rb") as f:
    code = f.read()

# remove prologue
i = 0
for i, byte in enumerate(code):
    if byte not in range(0x50, 0x58) and byte != 0x41:
        break
code = code[i:]

rem = len(code) % 8
if rem:
    code += b"\x00" * (8 - rem)

arr = array.array('Q')
arr.frombytes(code)
r = ",".join(min(str(i), hex(i), key=len) for i in arr)

with open(template_path) as f:
    template = f.read().rstrip()
print(template % {"len": len(arr), "text": r})
