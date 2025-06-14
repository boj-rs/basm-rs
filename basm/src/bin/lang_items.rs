#[unsafe(no_mangle)]
extern "C" fn _basm_main() {
    crate::solution::main()
}

#[cfg(not(test))]
mod runtime {
    #[global_allocator]
    static ALLOC: basm::platform::allocator::Allocator = basm::platform::allocator::Allocator;

    #[unsafe(no_mangle)]
    #[cfg(target_os = "windows")]
    static mut _fltused: i32 = 0;

    #[unsafe(no_mangle)]
    #[cfg(target_os = "windows")]
    extern "win64" fn __CxxFrameHandler3() -> ! {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[unsafe(no_mangle)]
    fn rust_eh_personality() {
        unsafe { core::hint::unreachable_unchecked() }
    }

    #[panic_handler]
    fn panic(_pi: &core::panic::PanicInfo) -> ! {
        #[cfg(not(feature = "submit"))]
        unsafe {
            basm::platform::codegen::print_panicinfo_and_exit(_pi)
        }
        #[cfg(feature = "submit")]
        unsafe {
            core::hint::unreachable_unchecked()
        }
    }
}
