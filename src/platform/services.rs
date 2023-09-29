static mut SERVICE_FUNCTIONS: usize = 0;

#[cfg(target_arch = "x86_64")]
pub mod native_func {
    pub type A = unsafe extern "win64" fn(usize, usize) -> *mut u8;
    pub type B = unsafe extern "win64" fn(*mut u8, usize, usize);
    pub type C = unsafe extern "win64" fn(*mut u8, usize, usize, usize) -> *mut u8;
    pub type D = unsafe extern "win64" fn(usize) -> !;
    pub type E = unsafe extern "win64" fn(usize, *mut u8, usize) -> usize;
    pub type F = unsafe extern "win64" fn(usize, *const u8, usize) -> usize;
}
#[cfg(not(target_arch = "x86_64"))]
pub mod native_func {
    pub type A = unsafe extern "C" fn(usize, usize) -> *mut u8;
    pub type B = unsafe extern "C" fn(*mut u8, usize, usize);
    pub type C = unsafe extern "C" fn(*mut u8, usize, usize, usize) -> *mut u8;
    pub type D = unsafe extern "C" fn(usize) -> !;
    pub type E = unsafe extern "C" fn(usize, *mut u8, usize) -> usize;
    pub type F = unsafe extern "C" fn(usize, *const u8, usize) -> usize;
}


pub const ENV_ID_UNKNOWN: u64 = 0;
pub const ENV_ID_WINDOWS: u64 = 1;
pub const ENV_ID_LINUX: u64 = 2;
pub const ENV_FLAGS_LINUX_STYLE_CHKSTK: u64 = 0x0001;   // disables __chkstk in binaries compiled with Windows target
pub const ENV_FLAGS_NATIVE: u64 = 0x0002;               // indicates the binary is running without the loader

#[repr(packed)]
#[allow(non_snake_case)]
pub struct PlatformData {
    pub env_id: u64,
    pub env_flags: u64,
    pub leading_unused_bytes: u64,
    pub pe_image_base: u64,
    pub pe_off_reloc: u64,
    pub pe_size_reloc: u64,
    pub win_GetModuleHandleW: u64,      // pointer to kernel32::GetModuleHandleA
    pub win_GetProcAddress: u64,        // pointer to kernel32::GetProcAddress
}

#[inline(always)]
pub fn install(service_functions_by_loader: usize) {
    unsafe {
        SERVICE_FUNCTIONS = service_functions_by_loader;
    }
}
#[inline(always)]
unsafe fn addr(fn_id: usize) -> usize {
    core::ptr::read((SERVICE_FUNCTIONS + fn_id * core::mem::size_of::<usize>()) as *mut usize)
}
#[inline(always)]
pub unsafe fn install_single_service(fn_id: usize, fn_ptr: usize) {
    core::ptr::write((SERVICE_FUNCTIONS + fn_id * core::mem::size_of::<usize>()) as *mut usize, fn_ptr)
}
//#[inline(always)]
pub unsafe fn alloc(size: usize, align: usize) -> *mut u8 {
    let fn_ptr: native_func::A = core::mem::transmute(addr(1));
    fn_ptr(size, align)
}
//#[inline(always)]
pub unsafe fn alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let fn_ptr: native_func::A = core::mem::transmute(addr(2));
    fn_ptr(size, align)
}
//#[inline(always)]
pub unsafe fn dealloc(ptr: *mut u8, size: usize, align: usize) {
    let fn_ptr: native_func::B = core::mem::transmute(addr(3));
    fn_ptr(ptr, size, align)
}
//#[inline(always)]
pub unsafe fn realloc(ptr: *mut u8, old_size: usize, old_align: usize, new_size: usize) -> *mut u8 {
    let fn_ptr: native_func::C = core::mem::transmute(addr(4));
    fn_ptr(ptr, old_size, old_align, new_size)
}
#[inline(always)]
pub fn exit(status: i32) -> ! {
    unsafe {
        let fn_ptr: native_func::D = core::mem::transmute(addr(5));
        fn_ptr(status as isize as usize)
    }
}
#[inline(always)]
pub fn read_stdio(fd: usize, buf: &mut [u8]) -> usize {
    unsafe {
        let fn_ptr: native_func::E = core::mem::transmute(addr(6));
        fn_ptr(fd, buf.as_mut_ptr(), buf.len())
    }
}
#[inline(always)]
pub fn write_stdio(fd: usize, buf: &[u8]) -> usize {
    unsafe {
        let fn_ptr: native_func::F = core::mem::transmute(addr(7));
        fn_ptr(fd, buf.as_ptr(), buf.len())
    }
}
#[inline(always)]
pub fn platform_data() -> *const PlatformData {
    unsafe {
        core::mem::transmute(addr(9))
    }
}