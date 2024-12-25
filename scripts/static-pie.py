import argparse
import os
import subprocess
import sys

def get_args_parser():
    parser = argparse.ArgumentParser("basm static-pie generator")
    parser.add_argument("--target", type=str, help="Rust target triple", required=True)
    parser.add_argument("--lang", type=str, choices=["C", "CFnImpl", "Rust", "JavaScript", "HTML"], required=True)
    parser.add_argument("--profile", type=str, choices=["Debug", "Release"], required=True)
    parser.add_argument("--cargo_args", nargs=argparse.REMAINDER)
    return parser

def run_cmds(cmds):
    for cmd in cmds:
        completed_process = subprocess.run(cmd)
        if completed_process.returncode != 0:
            print(cmd)
            raise Exception("Build command terminated with a non-zero exit code {}.".format(completed_process.returncode))

def main(args):
    # TODO: devise way to handle short
    cmds = []

    target = args.target
    lang = args.lang
    profile = args.profile
    profile_dir = {"Debug": "debug", "Release": "release"}[profile]
    cargo_args = args.cargo_args

    cargo_target_dir = os.environ.get("CARGO_TARGET_DIR", "target")
    short = any("short" in x for x in cargo_args)
    python = "python" if os.name == "nt" else "python3"

    if target == "x86_64-unknown-linux-gnu":
        stub = "static-pie-stub-amd64.bin"
        if lang == "C":
            if short:
                template = "static-pie-template-amd64-short.c"
            else:
                template = "static-pie-template-amd64.c"
        elif lang == "CFnImpl":
            template = "static-pie-template-amd64-fn-impl.cpp"
        elif lang == "Rust":
            if short:
                template = "static-pie-template-amd64-short.rs"
            else:
                template = "static-pie-template-amd64.rs"
        else:
            print(f"Language ${lang} is not supported for target ${target}", file=sys.stderr)
            exit()
    elif target == "i686-unknown-linux-gnu":
        stub = "static-pie-stub-i686.bin"
        if lang == "C":
            template = "static-pie-template-i686.c"
        else:
            print(f"Language ${lang} is not supported for target ${target}", file=sys.stderr)
            exit()
    elif target in ["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"]:
        stub = "static-pie-stub-amd64.bin"
        if lang == "C":
            if short:
                template = "static-pie-template-amd64-short.c"
            else:
                template = "static-pie-template-amd64.c"
        elif lang == "CFnImpl":
            template = "static-pie-template-amd64-fn-impl.cpp"
        elif lang == "Rust":
            if short:
                template = "static-pie-template-amd64-short.rs"
            else:
                template = "static-pie-template-amd64.rs"
        else:
            print(f"Language ${lang} is not supported for target ${target}", file=sys.stderr)
            exit()
    elif target == "wasm32-unknown-unknown":
        if lang == "JavaScript":
            template = "wasm-template.js"
        elif lang == "HTML":
            template = "wasm-template.html"
        else:
            print(f"Language ${lang} is not supported for target ${target}", file=sys.stderr)
            exit()
    else:
        print(f"Unknown target {target}", file=sys.stderr)
        exit()

    if target == "x86_64-unknown-linux-gnu" and short:
        target_cargo = ".cargo/x86_64-unknown-linux-gnu-short.json"
        target = "x86_64-unknown-linux-gnu-short"
        extra_config = ["-Zbuild-std=core,compiler_builtins,alloc", "-Zbuild-std-features=compiler-builtins-mem"]
    else:
        target_cargo = target
        extra_config = []

    if lang == "CFnImpl":
        lang = "C"

    print(f"Building project for target {target}, language {lang}, profile {profile}", file=sys.stderr)

    if profile == "Debug":
        cmds.append(["cargo", "+nightly", "build"] + extra_config + ["--target", target_cargo, "--bin", "basm-submit", "--features=submit"] + cargo_args)
    else:
        cmds.append(["cargo", "+nightly", "build"] + extra_config + ["--target", target_cargo, "--bin", "basm-submit", "--features=submit", "--release"] + cargo_args)

    if target in ["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"]:
        cmds.append([python, "scripts/static-pie-gen.py", "basm/", target, f"{cargo_target_dir}/{target}/{profile_dir}/basm-submit.exe", stub, lang, template])
    elif target in "wasm32-unknown-unknown":
        cmds.append([python, "scripts/wasm-gen.py", template, lang])
    else:        
        cmds.append(["cp", f"{cargo_target_dir}/{target}/{profile_dir}/basm-submit", f"{cargo_target_dir}/{target}/{profile_dir}/basm-submit-stripped"])
        cmds.append(["objcopy", "--strip-all", f"{cargo_target_dir}/{target}/{profile_dir}/basm-submit-stripped"])
        cmds.append(["objcopy", "--remove-section", ".eh_frame", "--remove-section", ".gcc_except_table", "--remove-section", ".gnu.hash", f"{cargo_target_dir}/{target}/{profile_dir}/basm-submit-stripped"])
        cmds.append([python, "scripts/static-pie-gen.py", "basm/", target, f"{cargo_target_dir}/{target}/{profile_dir}/basm-submit-stripped", stub, lang, template])

    run_cmds(cmds)

if __name__ == "__main__":
    parser = get_args_parser()
    args = parser.parse_args()
    main(args)