import base64
import io
import os
import re
import sys
import zlib

# solution_src
with open("src/solution.rs", encoding='utf8') as f:
    sol = f.readlines()

sol_all = "".join(sol)
sol_has_block_comment = "/*" in sol_all or "*/" in sol_all
if sol_has_block_comment:
    prefix, begin, end = "//", "", ""
else:
    prefix, begin, end = "", "/*\n", "*/\n"
sol = [line.replace("\ufeff", "") for line in sol]
sol = [prefix + line.rstrip() + "\n" for line in sol]
if len(begin) > 0:
    sol = [begin] + sol + [end]
if len(sol) > 0:
    sol[-1] = sol[-1].rstrip()
sol = "".join(sol)

# binary
with open("target/wasm32-unknown-unknown/release/basm-submit.wasm", "rb") as f:
    code = f.read()
cobj = zlib.compressobj(level=9, wbits=-15)
code = cobj.compress(code)
code += cobj.flush()
code = base64.b64encode(code).decode('ascii')

# template
with open(sys.argv[1], "r") as f:
    template = f.read()

# putting it all together
# reference: https://stackoverflow.com/a/15448887
def multiple_replace(string, rep_dict):
    pattern = re.compile("|".join([re.escape(k) for k in sorted(rep_dict,key=len,reverse=True)]), flags=re.DOTALL)
    return pattern.sub(lambda x: rep_dict[x.group(0)], string)

out = multiple_replace(template, {
    "$$$$solution_src$$$$": sol,
    "$$$$binary_base64$$$$": code,
})
print(out)