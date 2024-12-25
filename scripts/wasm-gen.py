import base64
import io
import locator
import os
import re
import srcpack
import sys
import utils
import zlib

# Utility function for compression
def deflate_raw(input_bytes):
    cobj = zlib.compressobj(level=9, wbits=-15)
    output_bytes = cobj.compress(input_bytes)
    output_bytes += cobj.flush()
    return output_bytes

# solution_src
solution_src_path = sys.argv[1]
target_language = sys.argv[3]
sol = srcpack.read_assemble(solution_src_path, target_language)
sol_b64 = base64.b64encode(deflate_raw(sol.encode('utf8'))).decode('ascii')

# binary
target = os.environ.get("CARGO_TARGET_DIR", "target")
if len(target) == 0:
    target = "target"
with open(f"{target}/wasm32-unknown-unknown/release/basm-submit.wasm", "rb") as f:
    code = f.read()
code = base64.b64encode(deflate_raw(code)).decode('ascii')

# template
with open(locator.template_path(sys.argv[2]), "r") as f:
    template = f.read()

out = utils.multiple_replace(template, {
    "$$$$solution_src$$$$": sol,
    "$$$$solution_src_base64$$$$": sol_b64,
    "$$$$binary_base64$$$$": code,
})
print(out)