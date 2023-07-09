static mut SERVICE_FUNCTIONS: usize = 0;

type NativeFuncA = unsafe extern "C" fn(usize) -> *mut u8;
type NativeFuncB = unsafe extern "C" fn(*mut u8);
type NativeFuncC = unsafe extern "C" fn(*mut u8, usize) -> *mut u8;
type NativeFuncD = unsafe extern "C" fn(usize) -> !;
type NativeFuncE = unsafe extern "C" fn(usize, *mut u8, usize) -> usize;
type NativeFuncF = unsafe extern "C" fn(usize, *const u8, usize) -> usize;

#[inline(always)]
pub fn init(service_functions_by_loader: usize) {
    unsafe {
        SERVICE_FUNCTIONS = service_functions_by_loader;
    }
}
#[inline(always)]
unsafe fn addr(fn_id: usize) -> usize {
    core::ptr::read((SERVICE_FUNCTIONS + fn_id * core::mem::size_of::<usize>()) as *mut usize)
}
#[inline(always)]
pub unsafe fn alloc(size: usize) -> *mut u8 {
    let fn_ptr: NativeFuncA = core::mem::transmute(addr(0));
    fn_ptr(size)
}
#[inline(always)]
pub unsafe fn alloc_zeroed(size: usize) -> *mut u8 {
    let fn_ptr: NativeFuncA = core::mem::transmute(addr(1));
    fn_ptr(size)
}
#[inline(always)]
pub unsafe fn dealloc(ptr: *mut u8) {
    let fn_ptr: NativeFuncB = core::mem::transmute(addr(2));
    fn_ptr(ptr)
}
#[inline(always)]
pub unsafe fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let fn_ptr: NativeFuncC = core::mem::transmute(addr(3));
    fn_ptr(ptr, new_size)
}
#[inline(always)]
pub fn exit(status: i32) -> ! {
    unsafe {
        let fn_ptr: NativeFuncD = core::mem::transmute(addr(4));
        fn_ptr(status as usize)
    }
}
#[inline(always)]
pub fn read_stdio(fd: usize, buf: &mut [u8]) -> usize {
    unsafe {
        let fn_ptr: NativeFuncE = core::mem::transmute(addr(5));
        fn_ptr(fd, buf.as_mut_ptr(), buf.len())
    }
}
#[inline(always)]
pub fn write_stdio(fd: usize, buf: &[u8]) -> usize {
    unsafe {
        let fn_ptr: NativeFuncF = core::mem::transmute(addr(6));
        fn_ptr(fd, buf.as_ptr(), buf.len())
    }
}