use core::alloc::GlobalAlloc;
use core::arch::asm;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let out;
        asm!(
            "syscall",
            in("rax") 9,
            in("rdi") 0,
            in("rsi") layout.size(),
            in("rdx") 0x3,
            in("r10") 0x22,
            in("r8") -1,
            in("r9") 0,
            lateout("rax") out,
            out("rcx") _,
            out("r11") _,
        );
        out
    }
    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.alloc(layout)
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        asm!(
            "syscall",
            in("rax") 11,
            in("rdi") ptr,
            in("rsi") layout.size(),
            lateout("rax") _,
            out("rcx") _,
            out("r11") _,
        );
    }
    #[inline(always)]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let out;
        asm!(
            "syscall",
            in("rax") 25,
            in("rdi") ptr,
            in("rsi") layout.size(),
            in("rdx") new_size,
            in("r10") 1,
            in("r8") 0,
            lateout("rax") out,
            out("rcx") _,
            out("r11") _,
        );
        out
    }
}
