#[no_mangle]
extern "C" fn _basm_main() {
    crate::solution::main()
}

#[cfg(not(test))]
mod runtime {
    #[global_allocator]
    static ALLOC: basm::platform::allocator::Allocator = basm::platform::allocator::Allocator;

    #[no_mangle]
    #[cfg(target_os = "windows")]
    static mut _fltused: i32 = 0;

    #[no_mangle]
    #[cfg(target_os = "windows")]
    extern "win64" fn __CxxFrameHandler3() -> ! {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[alloc_error_handler]
    fn alloc_fail(_: core::alloc::Layout) -> ! {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    fn _Unwind_Resume() {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[no_mangle]
    fn rust_eh_personality() {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[cfg(all(not(test), feature = "submit"))]
    #[panic_handler]
    fn panic(_pi: &core::panic::PanicInfo) -> ! {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[cfg(all(not(test), not(feature = "submit")))]
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
}