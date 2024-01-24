import os

# Reads and assembles the source code in the crate at the path `crate_root`.
# `crate_root` usually equals `basm/`.
def read_assemble(crate_root, target_language):
    sol_all = []
    crate_src_path = os.path.join(crate_root, "src/")
    for root, dirs, files in os.walk(crate_src_path):
        if os.path.abspath(root).startswith(os.path.abspath(os.path.join(crate_src_path, "bin/"))):
            continue
        for f in files:
            f_path = os.path.join(root, f)
            if f_path.endswith(".rs"):
                with open(f_path, encoding='utf8') as f:
                    sol = f.readlines()
                sol_all.append((f_path, sol))
    if len(sol_all) == 1:
        sol_flat = sol_all[0][1]
    else:
        sol_flat = []
        for i, (f_path, sol) in enumerate(sol_all):
            if i > 0:
                sol_flat.append("\n")
            sol_flat.append("// {0}\n".format(f_path))
            sol_flat.extend(sol)
    if target_language in ["Rust", "HTML"]:
        return assemble_as_is(sol_flat)
    else:
        return assemble_with_commenting(sol_flat)

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