#![feature(fn_align)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]
#![cfg_attr(not(test), no_builtins)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

extern crate basm_std as basm;

#[cfg_attr(test, allow(dead_code))]
#[path = "../solution.rs"]
mod solution;
mod codegen;

#[cfg(not(test))]
#[panic_handler]
fn panic(_pi: &core::panic::PanicInfo) -> ! {
    use alloc::string::ToString;
    use basm::platform::services::write_stdio;
    write_stdio(2, _pi.to_string().as_bytes());
    write_stdio(2, b"\n");

    // Rust sets an exit code of 101 when the process panicked.
    // Hence, we follow that practice for maximum compatibility.
    // Reference: https://rust-cli.github.io/book/in-depth/exit-code.html
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        extern "win64" {
            fn ExitProcess(uExitCode: u32) -> !;
        }
        unsafe { ExitProcess(101); }
    }
    #[cfg(target_os = "linux")] {
        unsafe { basm::platform::os::linux::syscall::exit_group(101); }
    }
    #[cfg(not(any(all(windows, target_arch = "x86_64"), target_os = "linux")))]
    unsafe { core::hint::unreachable_unchecked() }
}