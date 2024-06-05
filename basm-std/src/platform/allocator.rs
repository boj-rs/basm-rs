use core::alloc::GlobalAlloc;

static mut PTR_ALLOC: unsafe fn(usize, usize) -> *mut u8 = super::services::alloc;
static mut PTR_ALLOC_ZEROED: unsafe fn(usize, usize) -> *mut u8 = super::services::alloc_zeroed;
static mut PTR_DEALLOC: unsafe fn(*mut u8, usize, usize) = super::services::dealloc;
static mut PTR_REALLOC: unsafe fn(*mut u8, usize, usize, usize) -> *mut u8 =
    super::services::realloc;

pub unsafe fn install_malloc_impl(
    ptr_alloc: unsafe fn(usize, usize) -> *mut u8,
    ptr_alloc_zeroed: unsafe fn(usize, usize) -> *mut u8,
    ptr_dealloc: unsafe fn(*mut u8, usize, usize),
    ptr_realloc: unsafe fn(*mut u8, usize, usize, usize) -> *mut u8,
) {
    PTR_ALLOC = ptr_alloc;
    PTR_ALLOC_ZEROED = ptr_alloc_zeroed;
    PTR_DEALLOC = ptr_dealloc;
    PTR_REALLOC = ptr_realloc;
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        PTR_ALLOC(layout.size(), layout.align())
    }
    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        PTR_ALLOC_ZEROED(layout.size(), layout.align())
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        PTR_DEALLOC(ptr, layout.size(), layout.align())
    }
    #[inline(always)]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        PTR_REALLOC(ptr, layout.size(), layout.align(), new_size)
    }
}
