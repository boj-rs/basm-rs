use super::super::allocator;
use super::super::malloc::{dlmalloc, dlmalloc_linux};


static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_linux::System> = dlmalloc::Dlmalloc::new(dlmalloc_linux::System::new());
unsafe fn dlmalloc_alloc(size: usize) -> *mut u8 {
    DLMALLOC.malloc(size)
}
unsafe fn dlmalloc_alloc_zeroed(size: usize) -> *mut u8 {
    let ptr = DLMALLOC.malloc(size);
    if !ptr.is_null() && DLMALLOC.calloc_must_clear(ptr) {
        core::ptr::write_bytes(ptr, 0, size);
    }
    ptr
}
unsafe fn dlmalloc_dealloc(ptr: *mut u8) {
    DLMALLOC.free(ptr);
}
unsafe fn dlmalloc_realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    DLMALLOC.realloc(ptr, new_size)
}

pub unsafe fn init() {
    allocator::install_malloc_impl(
        dlmalloc_alloc,
        dlmalloc_alloc_zeroed,
        dlmalloc_dealloc,
        dlmalloc_realloc,
    );
}