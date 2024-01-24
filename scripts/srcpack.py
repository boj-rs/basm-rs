import os

# Reads and assembles the source code in the crate at the path `crate_root`.
# `crate_root` usually equals `basm/`.
def read_assemble(crate_root, target_language):
    solution_src_path = os.path.join(crate_root, "src/solution.rs")
    with open(solution_src_path, encoding='utf8') as f:
        sol = f.readlines()
    if target_language in ["Rust", "HTML"]:
        return assemble_as_is(sol)
    else:
        return assemble_with_commenting(sol)

def assemble_as_is(sol):
    sol = [line.replace("\ufeff", "") for line in sol]
    sol = [line.rstrip() + "\n" for line in sol]
    if len(sol) > 0:
        sol[-1] = sol[-1].rstrip()
    sol = "".join(sol)
    return sol

def assemble_with_commenting(sol):
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
    return sol