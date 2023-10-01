"""
This script builds and tests a solution for a problem.
Developed for use in CI.
Usage:
    python .\scripts\build-and-judge.py [tmp-dir] [build-cmd] [sol-path] [indata-path] [outdata-path]
Example:
    python .\scripts\build-and-judge.py .\scripts\tmp .\release-64bit-windows-rs.cmd Rust .\tests\boj_3745.rs .\tests\boj_3745.in .\tests\boj_3745.out

Limitations: special judges are not yet supported.
"""

import os
import platform
import shutil
import subprocess
import sys

def try_remove(filename):
    try:
        os.remove(filename)
    except OSError:
        pass

def test_equal(x, y):
    x_tok = str(x).split()
    y_tok = str(y).split()
    return x_tok == y_tok

if __name__ == '__main__':
    tmp_dir = sys.argv[1]
    build_cmd = sys.argv[2]
    language = sys.argv[3]
    bits = int(sys.argv[4])
    sol_path = sys.argv[5]
    indata_path = sys.argv[6]
    outdata_path = sys.argv[7]
    src_ext = {"C": "c", "Rust": "rs"}[language]

    # Prepare environment
    os.makedirs(tmp_dir, exist_ok=True)
    src_path = os.path.abspath(os.path.join(tmp_dir, "output.{0}".format(src_ext)))
    bin_path = os.path.abspath(os.path.join(tmp_dir, "loader.exe" if platform.system() == "Windows" else "loader"))
    try_remove(src_path)
    try_remove(bin_path)

    # Read the input and output data in advance
    with open(indata_path, mode="r", encoding="utf8") as f:
        indata = f.read()
    with open(outdata_path, mode="r", encoding="utf8") as f:
        outdata = f.read()

    # Replace the solution
    shutil.copyfile(sol_path, "src/solution_new.rs")
    os.rename("src/solution.rs", "src/solution_old.rs")
    os.rename("src/solution_new.rs", "src/solution.rs")

    # Build the project to generate the source code
    try:
        p = subprocess.run([build_cmd], shell=True, capture_output=True, text=True)
        if p.returncode != 0:
            raise Exception("Build failed. The stderr:\n{0}".format(p.stderr))
        source_code = p.stdout
        with open(src_path, mode="w", encoding="utf8") as f:
            f.write(source_code)
        print(source_code)
    finally:
        # Restore the original solution
        try_remove("src/solution.rs")
        os.rename("src/solution_old.rs", "src/solution.rs")

    # Compile the source code
    if language == "C":
        if platform.system() == "Windows":
            os.system("cl {0} /F268435456 /Fe{1} /link /SUBSYSTEM:CONSOLE".format(src_path, bin_path))
        else:
            os.system("gcc -o {1} {2} {0}".format(src_path, bin_path, "-O3 -m32" if bits == 32 else "-O3"))
    else: # language == "Rust"
        if platform.system() == "Windows":
            os.system("rustc -C opt-level=3 -o {1} --crate-type=bin {0}".format(src_path, bin_path))
        else:
            os.system("rustc -C opt-level=3 -o {1} {0}".format(src_path, bin_path))

    # Run the binary
    with open(indata_path, mode="r", encoding="utf8") as f:
        stdout = subprocess.run([bin_path], shell=False, stdin=f, capture_output=True, text=True).stdout

    if test_equal(stdout, outdata):
        print("Program succeeded for input {0} and output {1}".format(indata_path, outdata_path))
    else:
        err_msg = "Program fails to print the correct output for input {0} and output {1}\n".format(indata_path, outdata_path)
        err_msg += "Input:\n{0}\nOutput (expected):\n{1}\nOutput (actual):\n{2}\n".format(indata[:1000], outdata[:1000], stdout[:1000])
        raise Exception(err_msg)