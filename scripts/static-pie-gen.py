import array
import base64
import sys

try:
    binary_path, template_path = sys.argv[1:]
except ValueError:
    print(f"Usage: {sys.argv[0]} binary_path template_path", file=sys.stderr)
    sys.exit(1)

with open(binary_path, "rb") as f:
    code = f.read()

code = bytearray(code)
while len(code) % 4 != 0:
    code.append(0)

r = base64.b85encode(code, pad=False) # already padded
r = r.decode('ascii')

with open(template_path, encoding='utf8') as f:
    template = f.read()
print(template.replace("$$$$len$$$$", str(len(code))).replace("$$$$binary_base85$$$$", r))
