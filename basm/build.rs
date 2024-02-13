use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut link_args_basm = vec![];
    let mut link_args_basm_submit = vec![];

    println!("cargo:rerun-if-changed=build.rs");
    match target.as_str() {
        "x86_64-pc-windows-msvc" => {
            link_args_basm.push("/SUBSYSTEM:CONSOLE");
            link_args_basm.push("/NODEFAULTLIB");
            link_args_basm.push("/DYNAMICBASE");
            link_args_basm.push("/ENTRY:_basm_start");
            link_args_basm.push("/NXCOMPAT:NO");
            link_args_basm.push("/STACK:268435456");
            link_args_basm.push("/EMITTOOLVERSIONINFO:NO");
            link_args_basm.push("/EMITPOGOPHASEINFO");
            link_args_basm_submit.push("/ALIGN:128");
            link_args_basm_submit.push("/OPT:REF,ICF");
        },
        "x86_64-unknown-linux-gnu" | "x86_64-unknown-linux-gnu-short" | "i686-unknown-linux-gnu" => {
            link_args_basm.push("-nostartfiles");
            link_args_basm.push("-nostdlib");
            link_args_basm.push("-static-pie");
            link_args_basm.push("-fno-exceptions");
            link_args_basm.push("-fno-asynchronous-unwind-tables");
            link_args_basm.push("-fno-unwind-tables");
            link_args_basm.push("-fno-stack-protector");
            link_args_basm.push("-fno-plt");
            if target == "i686-unknown-linux-gnu" {
                // Prevent linker from putting data into text, which is non-writable and hence not relocatable.
                // This prevents the hack for getting the _DYNAMIC symbol in the entrypoint.
                link_args_basm.push("-Wl,--entry=_basm_start,--build-id=none,--gc-sections,--export-dynamic,--no-eh-frame-hdr,-z,norelro,-z,notext");
            } else {
                link_args_basm.push("-Wl,--entry=_basm_start,--build-id=none,--gc-sections,--export-dynamic,--no-eh-frame-hdr,-z,norelro");
            }
            link_args_basm_submit.push("-Wl,-z,max-page-size=128");
        },
        "aarch64-apple-darwin" => {
            link_args_basm.push("-nostartfiles");
            link_args_basm.push("-nostdlib");
            link_args_basm.push("-fno-exceptions");
            link_args_basm.push("-fno-asynchronous-unwind-tables");
            link_args_basm.push("-fno-unwind-tables");
            link_args_basm.push("-fno-stack-protector");
            link_args_basm.push("-fno-plt");
            link_args_basm.push("-e__basm_start");
        },
        "wasm32-unknown-unknown" => {
        },
        _ => {
            panic!("Unsupported target {target}");
        }
    }
    for s in link_args_basm {
        println!("cargo:rustc-link-arg-bin=basm={s}");
        println!("cargo:rustc-link-arg-bin=basm-submit={s}");
    }
    for s in link_args_basm_submit {
        println!("cargo:rustc-link-arg-bin=basm-submit={s}");
    }
}