import array
import base64
import re
import sys

try:
    solution_src_path, binary_path, template_path = sys.argv[1:]
except ValueError:
    print(f"Usage: {sys.argv[0]} solution_src_path binary_path template_path", file=sys.stderr)
    sys.exit(1)

# solution_src
with open(solution_src_path, encoding='utf8') as f:
    sol = f.readlines()

sol = [line.replace("\ufeff", "") for line in sol]
sol = ["//" + line.rstrip() + "\n" for line in sol]
if len(sol) > 0:
    sol[-1] = sol[-1].rstrip()
sol = "".join(sol)

# binary
with open(binary_path, "rb") as f:
    code = f.read()

code = bytearray(code)
while len(code) % 4 != 0:
    code.append(0)

r = base64.b85encode(code, pad=False) # already padded
r = r.decode('ascii').replace("?", "\?")

# template
with open(template_path, encoding='utf8') as f:
    template = f.read()

# putting it all together
# reference: https://stackoverflow.com/a/15448887
def multiple_replace(string, rep_dict):
    pattern = re.compile("|".join([re.escape(k) for k in sorted(rep_dict,key=len,reverse=True)]), flags=re.DOTALL)
    return pattern.sub(lambda x: rep_dict[x.group(0)], string)

out = multiple_replace(template, {"$$$$solution_src$$$$": sol, "$$$$len$$$$": str(len(code)), "$$$$binary_base85$$$$": r})
print(out)