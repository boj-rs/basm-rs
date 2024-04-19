"""
This script builds and tests a solution for a problem.
Developed for use in CI.
Usage:
    python .\scripts\build-and-judge.py [tmp-dir] [build-cmd] [language] [bits] [sol-path] [indata-path] [outdata-path]
Example:
    python .\scripts\build-and-judge.py .\tmp\test\ .\release-64bit-windows-rs.cmd Rust 64 .\tests\boj_3745.rs .\tests\boj_3745.in .\tests\boj_3745.out

Limitations: special judges are not yet supported.
"""

import json
import os
import platform
import shutil
import subprocess
import sys
import zipfile

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
    build_cmd = [sys.argv[2]]
    language = sys.argv[3]
    bits = int(sys.argv[4])
    sol_path = sys.argv[5]
    indata_path = sys.argv[6]
    outdata_path = sys.argv[7]
    src_ext = {"Cargo": "cargo", "C": "c", "Rust": "rs", "JavaScript": "js"}[language]
    if language == "Cargo":
        build_cmd = ["cargo build --release --message-format=json"]
        if platform.system() == "Windows":
            build_cmd = build_cmd[0].split()

    # Prepare environment
    os.makedirs(tmp_dir, exist_ok=True)
    src_path = os.path.abspath(os.path.join(tmp_dir, "output.{0}".format(src_ext)))
    bin_path = os.path.abspath(os.path.join(tmp_dir, "loader.exe" if platform.system() == "Windows" else "loader"))
    try_remove(src_path)
    try_remove(bin_path)

    # Read the input and output data in advance
    if indata_path.endswith(".zip"):
        d = os.path.dirname(indata_path)
        with zipfile.ZipFile(indata_path, 'r') as zip_ref:
            zip_ref.extractall(d)
        indata_path = indata_path[:-4]
    with open(indata_path, mode="r", encoding="utf8") as f:
        indata = f.read()
    if outdata_path.endswith(".zip"):
        d = os.path.dirname(outdata_path)
        with zipfile.ZipFile(outdata_path, 'r') as zip_ref:
            zip_ref.extractall(d)
        outdata_path = outdata_path[:-4]
    with open(outdata_path, mode="r", encoding="utf8") as f:
        outdata = f.read()

    # Replace the solution
    shutil.copyfile(sol_path, "basm/src/solution_new.rs")
    os.rename("basm/src/solution.rs", "basm/src/solution_old.rs")
    os.rename("basm/src/solution_new.rs", "basm/src/solution.rs")

    # Build the project to generate the source code
    try:
        p = subprocess.run(build_cmd, shell=True, capture_output=True, text=True, encoding="utf8")
        if p.returncode != 0:
            raise Exception("Build failed. The stderr:\n{0}".format(p.stderr))
        if language == "Cargo":
            print(p.stderr)
            cargo_stdout = [json.loads(x) for x in p.stdout.split('\n') if len(x.strip()) > 0]
            basm_executable_path = None
            for obj in cargo_stdout:
                if obj["reason"] == "compiler-artifact" \
                   and obj["target"]["name"] == "basm" \
                   and "bin" in obj["target"]["crate_types"]:
                    basm_executable_path = obj["executable"]
                    break
            assert basm_executable_path is not None
        else:
            source_code = p.stdout
            with open(src_path, mode="w", encoding="utf8") as f:
                f.write(source_code)
            print(source_code)
    finally:
        # Restore the original solution
        try_remove("basm/src/solution.rs")
        os.rename("basm/src/solution_old.rs", "basm/src/solution.rs")

    # Compile the source code
    run_cmd = [bin_path]
    if language == "C":
        if platform.system() == "Windows":
            os.system("cl {0} /F268435456 /Fe{1} /link /SUBSYSTEM:CONSOLE".format(src_path, bin_path))
        else:
            os.system("gcc -DBASM_CI -o {1} {2} {0}".format(src_path, bin_path, "-O3 -m32" if bits == 32 else "-O3"))
    elif language == "Rust":
        if platform.system() == "Windows":
            os.system("rustc -C opt-level=3 -o {1} --crate-type=bin {0}".format(src_path, bin_path))
        else:
            os.system("rustc -C opt-level=3 -o {1} {0}".format(src_path, bin_path))
    elif language == "JavaScript":
        run_cmd = ["node", src_path]
    else: # language == "Cargo"
        if platform.system() != "Windows":
            os.system("chmod +x {0}".format(basm_executable_path))
        run_cmd = [basm_executable_path]

    # Run the binary
    with open(indata_path, mode="r", encoding="utf8") as f:
        completed_process = subprocess.run(run_cmd, shell=False, stdin=f, capture_output=True, text=True, encoding="utf8")
    if completed_process.returncode != 0:
        raise Exception("Program {0} exited with non-zero code {3} (hex {3:X}) for input {1} and output {2}\n\n{4}"
            .format(sol_path, indata_path, outdata_path, completed_process.returncode, completed_process.stderr))
    if test_equal(completed_process.stdout, outdata):
        print("Program {0} succeeded for input {1} and output {2}".format(sol_path, indata_path, outdata_path))
    else:
        err_msg = "Program {0} fails to print the correct output for input {1} and output {2}\n".format(sol_path, indata_path, outdata_path)
        err_msg += "Input:\n{0}\nOutput (expected):\n{1}\nOutput (actual):\n{2}\n".format(indata[:1000], outdata[:1000], completed_process.stdout[:1000])
        raise Exception(err_msg)
