"""
This script builds and tests solutions to multiple problems.
Developed for use in CI.
Usage:
    python scripts/ci.py [tmp-dir] [build-cmd] [language] [bits] [ci-json-path]
Example:
    python scripts/ci.py tmp/test release-64bit-windows-rs.cmd Rust 64 tests/ci.json
"""

import json
import os
import platform
import subprocess
import sys

if __name__ == "__main__":
    with open(sys.argv[5], "r") as f:
        ci_jobs = json.load(f)

    try:
        tmp_dir = sys.argv[1]
        build_cmd = sys.argv[2]
        language = sys.argv[3]
        bits = sys.argv[4]
    except:
        raise Exception("\n".join([
            "",
            "",
            "**Error: incorrect argument**",
            "",
            "This script builds and tests solutions to multiple problems.",
            "Developed for use in CI.",
            "Usage:",
            "    python scripts/ci.py [tmp-dir] [build-cmd] [language] [bits] [ci-json-path]",
            "Example:",
            "    python scripts/ci.py tmp/test release-64bit-windows-rs.cmd Rust 64 tests/ci.json"
        ]))

    skipped_count = 0
    for job in ci_jobs:
        sol_path = job["solution"]
        indata_path = job["input"]
        outdata_path = job["output"]
        if "target_platforms" in job:
            matched = False
            for d in job["target_platforms"]:
                if "os" in d and d["os"] != platform.system():
                    continue
                if "language" in d and d["language"] != language:
                    continue
                if "bits" in d and d["bits"] != bits:
                    continue
                matched = True
                break
            if not matched:
                skipped_count += 1
                continue
        completed_process = subprocess.run(" ".join([
            "python" if os.name == 'nt' else "python3",
            "./scripts/build-and-judge.py",
            tmp_dir,
            build_cmd,
            language,
            bits,
            sol_path,
            indata_path,
            outdata_path
        ]), shell=True, capture_output=False, text=True)
        if completed_process.returncode != 0:
            raise Exception("Test script terminated with a non-zero exit code {}.".format(completed_process.returncode))

    print("--------")
    jobs = "jobs" if len(ci_jobs) > 1 else "job"
    print(f"Successfully completed {len(ci_jobs)} {jobs} for [{build_cmd} {language} {bits}] (tmp dir: {tmp_dir}, skipped: {skipped_count})")