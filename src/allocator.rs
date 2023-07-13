use core::alloc::GlobalAlloc;
use crate::services;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        services::alloc(layout.size())
    }
    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        services::alloc_zeroed(layout.size())
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        services::dealloc(ptr)
    }
    #[inline(always)]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        _layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        services::realloc(ptr, new_size)
    }
}
