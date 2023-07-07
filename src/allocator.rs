use core::alloc::GlobalAlloc;

pub struct Allocator;

type NativeFuncA = unsafe extern "C" fn(usize) -> *mut u8;
type NativeFuncB = unsafe extern "C" fn(*mut u8);
type NativeFuncC = unsafe extern "C" fn(*mut u8, usize) -> *mut u8;

static mut PTR_ALLOC: Option<NativeFuncA> = None;
static mut PTR_ALLOC_ZEROED: Option<NativeFuncA> = None;
static mut PTR_DEALLOC: Option<NativeFuncB> = None;
static mut PTR_REALLOC: Option<NativeFuncC> = None;

impl Allocator {
    pub unsafe fn init(&self, service_functions: usize) {
        PTR_ALLOC           = core::mem::transmute(core::ptr::read((service_functions + 0 * core::mem::size_of::<usize>()) as *mut usize));
        PTR_ALLOC_ZEROED    = core::mem::transmute(core::ptr::read((service_functions + 1 * core::mem::size_of::<usize>()) as *mut usize));
        PTR_DEALLOC         = core::mem::transmute(core::ptr::read((service_functions + 2 * core::mem::size_of::<usize>()) as *mut usize));
        PTR_REALLOC         = core::mem::transmute(core::ptr::read((service_functions + 3 * core::mem::size_of::<usize>()) as *mut usize));
    }
}

unsafe impl GlobalAlloc for Allocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        PTR_ALLOC.unwrap()(layout.size())
    }
    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        PTR_ALLOC_ZEROED.unwrap()(layout.size())
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        PTR_DEALLOC.unwrap()(ptr)
    }
    #[inline(always)]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        _layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        PTR_REALLOC.unwrap()(ptr, new_size)
    }
}
