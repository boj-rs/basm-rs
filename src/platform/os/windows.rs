use super::super::{allocator, services};
use super::super::malloc::{dlmalloc, dlmalloc_windows};

#[cfg(target_arch = "x86_64")]
#[allow(non_snake_case)]
pub struct WinApi {
    kernel32: usize,
    ptr_GetModuleHandleW: Option<extern "win64" fn(*const u16) -> usize>,
    ptr_GetProcAddress: Option<extern "win64" fn(usize, *const u8) -> usize>,
    ptr_VirtualAlloc: Option<extern "win64" fn(*mut u8, usize, u32, u32) -> *mut u8>,
    ptr_VirtualFree: Option<extern "win64" fn(*mut u8, usize, u32) -> i32>,
}
#[cfg(target_arch = "x86")]
#[allow(non_snake_case)]
pub struct WinApi {
    kernel32: usize,
    ptr_GetModuleHandleW: Option<extern "stdcall" fn(*const u16) -> usize>,
    ptr_GetProcAddress: Option<extern "stdcall" fn(usize, *const u8) -> usize>,
    ptr_VirtualAlloc: Option<extern "stdcall" fn(*mut u8, usize, u32, u32) -> *mut u8>,
    ptr_VirtualFree: Option<extern "stdcall" fn(*mut u8, usize, u32) -> i32>,
}
#[allow(non_snake_case)]
impl WinApi {
    pub const KERNEL32DLL: [u16; 13] = [
        b'k' as u16, b'e' as u16, b'r' as u16, b'n' as u16, b'e' as u16, b'l' as u16,
        b'3' as u16, b'2' as u16, b'.' as u16, b'd' as u16, b'l' as u16, b'l' as u16,
        0u16];
    #[inline(always)]
    pub unsafe fn GetModuleHandleW(&self, lpModuleName: *const u16) -> usize {
        (self.ptr_GetModuleHandleW.unwrap())(lpModuleName)
    }
    #[inline(always)]
    pub unsafe fn GetProcAddress(&self, hModule: usize, lpProcName: *const u8) -> usize {
        (self.ptr_GetProcAddress.unwrap())(hModule, lpProcName)
    }
    #[inline(always)]
    pub unsafe fn VirtualAlloc(&self, lpAddress: *mut u8, dwSize: usize, flAllocationType: u32, flProtect: u32) -> *mut u8 {
        (self.ptr_VirtualAlloc.unwrap())(lpAddress, dwSize, flAllocationType, flProtect)
    }
    #[inline(always)]
    pub unsafe fn VirtualFree(&self, lpAddress: *mut u8, dwSize: usize, dwFreeType: u32) -> i32 {
        (self.ptr_VirtualFree.unwrap())(lpAddress, dwSize, dwFreeType)
    }
}
pub static mut WINAPI: WinApi = WinApi {
    kernel32: 0,
    ptr_GetModuleHandleW: None,
    ptr_GetProcAddress: None,
    ptr_VirtualAlloc: None,
    ptr_VirtualFree: None,
};

static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_windows::System> = dlmalloc::Dlmalloc::new(dlmalloc_windows::System::new());
unsafe fn dlmalloc_alloc(size: usize, align: usize) -> *mut u8 {
    DLMALLOC.memalign(align, size)
}
unsafe fn dlmalloc_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let ptr = DLMALLOC.memalign(align, size);
    if !ptr.is_null() && DLMALLOC.calloc_must_clear(ptr) {
        core::ptr::write_bytes(ptr, 0, size);
    }
    ptr
}
unsafe fn dlmalloc_dealloc(ptr: *mut u8, _size: usize, _align: usize) {
    DLMALLOC.free(ptr);
}
unsafe fn dlmalloc_realloc(ptr: *mut u8, old_size: usize, old_align: usize, new_size: usize) -> *mut u8 {
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

pub unsafe fn init() {
    let pd = services::platform_data();
    WINAPI.ptr_GetModuleHandleW = Some(core::mem::transmute((*pd).win_GetModuleHandleW as usize));
    WINAPI.ptr_GetProcAddress = Some(core::mem::transmute((*pd).win_GetProcAddress as usize));
    WINAPI.kernel32 = WINAPI.GetModuleHandleW(WinApi::KERNEL32DLL.as_ptr());
    WINAPI.ptr_VirtualAlloc = Some(core::mem::transmute(WINAPI.GetProcAddress(WINAPI.kernel32, b"VirtualAlloc\0".as_ptr() as *const u8)));
    WINAPI.ptr_VirtualFree = Some(core::mem::transmute(WINAPI.GetProcAddress(WINAPI.kernel32, b"VirtualFree\0".as_ptr() as *const u8)));

    allocator::install_malloc_impl(
        dlmalloc_alloc,
        dlmalloc_alloc_zeroed,
        dlmalloc_dealloc,
        dlmalloc_realloc,
    );

    /*
    let loadlibw: extern "win64" fn(*const u16) -> usize =
        core::mem::transmute(WINAPI.GetProcAddress(WINAPI.kernel32, b"LoadLibraryW\0".as_ptr() as *const u8));
    let user32 = loadlibw(b"u\0s\0e\0r\03\02\0.\0d\0l\0l\0\0\0".as_ptr() as *const u16);
    //let user32 = (WINAPI.GetModuleHandleW.unwrap())(b"u\0s\0e\0r\03\02\0.\0d\0l\0l\0\0\0".as_ptr() as *const u16);
    let msgboxw: extern "win64" fn(usize, *const u16, *const u16, u32) -> i32 =
        core::mem::transmute(WINAPI.GetProcAddress(user32, b"MessageBoxW\0".as_ptr() as *const u8));
    msgboxw(0, b"T\0\0\0".as_ptr() as *const u16, b"C\0\0\0".as_ptr() as *const u16, 0);
    */
}
