use super::super::malloc::{dlmalloc, dlmalloc_wasm32};
use super::super::{allocator, services};

static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_wasm32::System> =
    dlmalloc::Dlmalloc::new(dlmalloc_wasm32::System::new());
unsafe fn dlmalloc_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { DLMALLOC.memalign(align, size) }
}
unsafe fn dlmalloc_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    unsafe {
        let ptr = DLMALLOC.memalign(align, size);
        if !ptr.is_null() && DLMALLOC.calloc_must_clear(ptr) {
            core::ptr::write_bytes(ptr, 0, size);
        }
        ptr
    }
}
unsafe fn dlmalloc_dealloc(ptr: *mut u8, _size: usize, _align: usize) {
    unsafe {
        DLMALLOC.free(ptr);
    }
}
unsafe fn dlmalloc_realloc(
    ptr: *mut u8,
    old_size: usize,
    old_align: usize,
    new_size: usize,
) -> *mut u8 {
    unsafe {
        if old_align <= DLMALLOC.malloc_alignment() {
            DLMALLOC.realloc(ptr, new_size)
        } else {
            let ptr_new = DLMALLOC.memalign(old_align, new_size);
            if !ptr_new.is_null() {
                core::ptr::copy_nonoverlapping(ptr, ptr_new, core::cmp::min(old_size, new_size));
                DLMALLOC.free(ptr);
            }
            ptr_new
        }
    }
}

unsafe extern "C" {
    fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize;
    fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize;
}

pub unsafe fn init() {
    unsafe {
        allocator::install_malloc_impl(
            dlmalloc_alloc,
            dlmalloc_alloc_zeroed,
            dlmalloc_dealloc,
            dlmalloc_realloc,
        );
        services::install_single_service(5, svc_read_stdio as usize);
        services::install_single_service(6, svc_write_stdio as usize);
    }
}
