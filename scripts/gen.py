import array
import os
import sys

try:
    binary_path, template_path = sys.argv[1:]
except ValueError:
    print(f"Usage: {sys.argv[0]} binary_path template_path", file=sys.stderr)
    sys.exit(1)

code = bytearray((os.path.getsize(binary_path) + 7) // 8 * 8)
with open(binary_path, "rb") as f:
    f.readinto(code)

arr = array.array('Q')
arr.frombytes(code)
r = ",".join(min(str(i), hex(i), key=len) for i in arr)

with open(template_path) as f:
    template = f.read().rstrip()
print(template % r)
