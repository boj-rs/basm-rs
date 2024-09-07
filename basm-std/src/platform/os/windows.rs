#![allow(non_snake_case)]
#![allow(clippy::missing_transmute_annotations)]

use super::super::malloc::{dlmalloc, dlmalloc_windows};
use super::super::{allocator, services};

#[cfg(target_arch = "x86_64")]
macro_rules! ms_abi {
    (fn $args:tt) => { extern "win64" fn $args };
    (fn $args:tt $arrow: tt $rettype: ty) => { extern "win64" fn $args $arrow $rettype };
}
#[cfg(not(target_arch = "x86_64"))]
macro_rules! ms_abi {
    (fn $args:tt) => { extern "stdcall" fn $args };
    (fn $args:tt $arrow: tt $return_type: ty) => { extern "stdcall" fn $args $arrow $return_type };
}
// Note: to return nothing with basm_abi, use the equivalent form '-> ()'
#[cfg(target_arch = "x86_64")]
macro_rules! basm_abi {
    (fn $name:ident $args:tt -> $rettype:ty
         $code: block) =>
        {extern "win64" fn $name $args -> $rettype $code };
    ($s1:ident fn $name: ident $args:tt -> $rettype:ty $code: block) =>
        {$s1 extern "win64" fn $name $args -> $rettype $code };
    ($s1:ident $s2:ident fn $name: ident $args:tt -> $rettype:ty $code: block) =>
        {$s1 $s2 extern "win64" fn $name $args -> $rettype $code };
}
#[cfg(not(target_arch = "x86_64"))]
macro_rules! basm_abi {
    (fn $name:ident $args:tt -> $rettype:ty
         $code: block) =>
        {extern "C" fn $name $args -> $rettype $code };
    ($s1:ident fn $name: ident $args:tt -> $rettype:ty $code: block) =>
        {$s1 extern "C" fn $name $args -> $rettype $code };
    ($s1:ident $s2:ident fn $name: ident $args:tt -> $rettype:ty $code: block) =>
        {$s1 $s2 extern "C" fn $name $args -> $rettype $code };
}

#[repr(C)]
#[derive(Default)]
pub struct Overlapped {
    Internal: usize,
    InternalHigh: usize,
    Offset: u32,
    OffsetHigh: u32,
    hEvent: usize,
}
impl Overlapped {
    pub fn get_off(&self) -> u64 {
        ((self.OffsetHigh as u64) << 32) | (self.Offset as u64)
    }
    pub fn set_off(&mut self, off: u64) {
        self.Offset = off as u32;
        self.OffsetHigh = (off >> 32) as u32;
    }
}
#[derive(Default)]
pub struct WinApi {
    ptr_VirtualAlloc: Option<ms_abi! {fn(*mut u8, usize, u32, u32) -> *mut u8}>,
    ptr_VirtualFree: Option<ms_abi! {fn(*mut u8, usize, u32) -> i32}>,
    ptr_GetStdHandle: Option<ms_abi! {fn(u32) -> usize}>,
    ptr_ReadFile: Option<ms_abi! {fn(usize, *mut u8, u32, *mut u32, *mut Overlapped) -> i32}>,
    ptr_WriteFile: Option<ms_abi! {fn(usize, *const u8, u32, *mut u32, *mut Overlapped) -> i32}>,
    ptr_GetOverlappedResult: Option<ms_abi! {fn(usize, *mut Overlapped, *mut u32, i32) -> i32}>,
    ptr_GetLastError: Option<ms_abi! {fn() -> u32}>,
    io_off: [u64; 3],
}
impl WinApi {
    pub const KERNEL32: [u16; 9] = [
        b'k' as u16,
        b'e' as u16,
        b'r' as u16,
        b'n' as u16,
        b'e' as u16,
        b'l' as u16,
        b'3' as u16,
        b'2' as u16,
        0u16,
    ];
    pub const INVALID_HANDLE_VALUE: usize = -1isize as usize;
    pub const STD_INPUT_HANDLE: u32 = -10i32 as u32;
    pub const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
    pub const STD_ERROR_HANDLE: u32 = -12i32 as u32;
    pub const ERROR_IO_PENDING: u32 = 997;
    pub const CP_UTF8: u32 = 65001;
    #[inline(always)]
    pub unsafe fn VirtualAlloc(
        &self,
        lpAddress: *mut u8,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut u8 {
        (self.ptr_VirtualAlloc.unwrap())(lpAddress, dwSize, flAllocationType, flProtect)
    }
    #[inline(always)]
    pub unsafe fn VirtualFree(&self, lpAddress: *mut u8, dwSize: usize, dwFreeType: u32) -> i32 {
        (self.ptr_VirtualFree.unwrap())(lpAddress, dwSize, dwFreeType)
    }
    #[inline(always)]
    pub unsafe fn GetStdHandle(&self, nStdHandle: u32) -> usize {
        (self.ptr_GetStdHandle.unwrap())(nStdHandle)
    }
    #[inline(always)]
    pub unsafe fn ReadFile(
        &self,
        hFile: usize,
        lpBuffer: *mut u8,
        nNumberOfBytesToRead: u32,
        lpNumberOfBytesRead: *mut u32,
        lpOverlapped: *mut Overlapped,
    ) -> i32 {
        (self.ptr_ReadFile.unwrap())(
            hFile,
            lpBuffer,
            nNumberOfBytesToRead,
            lpNumberOfBytesRead,
            lpOverlapped,
        )
    }
    #[inline(always)]
    pub unsafe fn WriteFile(
        &self,
        hFile: usize,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: *mut u32,
        lpOverlapped: *mut Overlapped,
    ) -> i32 {
        (self.ptr_WriteFile.unwrap())(
            hFile,
            lpBuffer,
            nNumberOfBytesToWrite,
            lpNumberOfBytesWritten,
            lpOverlapped,
        )
    }
    #[inline(always)]
    pub unsafe fn GetOverlappedResult(
        &self,
        hFile: usize,
        lpOverlapped: *mut Overlapped,
        lpNumberOfBytesTransferred: *mut u32,
        bWait: i32,
    ) -> i32 {
        (self.ptr_GetOverlappedResult.unwrap())(
            hFile,
            lpOverlapped,
            lpNumberOfBytesTransferred,
            bWait,
        )
    }
    #[inline(always)]
    pub unsafe fn GetLastError(&self) -> u32 {
        (self.ptr_GetLastError.unwrap())()
    }
}
pub static mut WINAPI: WinApi = WinApi {
    ptr_VirtualAlloc: None,
    ptr_VirtualFree: None,
    ptr_GetStdHandle: None,
    ptr_ReadFile: None,
    ptr_WriteFile: None,
    ptr_GetOverlappedResult: None,
    ptr_GetLastError: None,
    io_off: [0; 3],
};

static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_windows::System> =
    dlmalloc::Dlmalloc::new(dlmalloc_windows::System::new());
unsafe fn dlmalloc_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe {
        DLMALLOC.memalign(align, size)
    }
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

mod services_override {
    use super::*;
    basm_abi! {pub unsafe fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
        unsafe {
            debug_assert!(fd == 0);
            let handle = WINAPI.GetStdHandle(WinApi::STD_INPUT_HANDLE);
            let mut bytes_read: u32 = 0;
            let mut ov: Overlapped = Default::default();
            ov.set_off(WINAPI.io_off[fd]);
            let mut ret = WINAPI.ReadFile(handle, buf, count as u32,
                &mut bytes_read as *mut u32, &mut ov as *mut Overlapped);
            if ret == 0 {
                let err = WINAPI.GetLastError();
                if err == WinApi::ERROR_IO_PENDING {
                    ret = WINAPI.GetOverlappedResult(handle, &mut ov as *mut Overlapped,
                        &mut bytes_read as *mut u32, 1);
                }
                if ret != 0 { return 0; }
            }
            WINAPI.io_off[fd] += bytes_read as u64;
            bytes_read as usize
        }
    }}
    basm_abi! {pub unsafe fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
            unsafe {
            debug_assert!(fd == 1 || fd == 2);
            let handle = match fd {
                1 => { WINAPI.GetStdHandle(WinApi::STD_OUTPUT_HANDLE) },
                2 => { WINAPI.GetStdHandle(WinApi::STD_ERROR_HANDLE) },
                _ => { unreachable!(); }
            };
            let mut bytes_written: u32 = 0;
            let mut ov: Overlapped = Default::default();
            ov.set_off(WINAPI.io_off[fd]);
            let mut ret = WINAPI.WriteFile(handle, buf, count as u32,
                &mut bytes_written as *mut u32, &mut ov as *mut Overlapped);
            if ret == 0 {
                let err = WINAPI.GetLastError();
                if err == WinApi::ERROR_IO_PENDING {
                    ret = WINAPI.GetOverlappedResult(handle, &mut ov as *mut Overlapped,
                        &mut bytes_written as *mut u32, 1);
                }
                if ret != 0 { return 0; }
            }
            WINAPI.io_off[fd] += bytes_written as u64;
            bytes_written as usize
        }
    }}
}

pub unsafe fn init() {
    unsafe {
        let pd = services::platform_data();
        let kernel32 = pd.win_kernel32 as usize;
        let GetProcAddress: ms_abi! {fn(usize, *const u8) -> usize} =
            core::mem::transmute(pd.win_GetProcAddress as usize);
        WINAPI.ptr_VirtualAlloc = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"VirtualAlloc\0".as_ptr(),
        )));
        WINAPI.ptr_VirtualFree = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"VirtualFree\0".as_ptr(),
        )));
        WINAPI.ptr_GetStdHandle = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"GetStdHandle\0".as_ptr(),
        )));
        WINAPI.ptr_ReadFile = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"ReadFile\0".as_ptr(),
        )));
        WINAPI.ptr_WriteFile = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"WriteFile\0".as_ptr(),
        )));
        WINAPI.ptr_GetOverlappedResult = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"GetOverlappedResult\0".as_ptr(),
        )));
        WINAPI.ptr_GetLastError = Some(core::mem::transmute(GetProcAddress(
            kernel32,
            b"GetLastError\0".as_ptr(),
        )));

        // On Windows, set console codepage to UTF-8,
        // since the default encoding is (historically) MBCS
        // which depends on the host platform's language and
        // other factors.
        let SetConsoleCP: ms_abi! {fn(u32) -> i32} =
            core::mem::transmute(GetProcAddress(kernel32, b"SetConsoleCP\0".as_ptr()));
        SetConsoleCP(WinApi::CP_UTF8); // for stdin
        let SetConsoleOutputCP: ms_abi! {fn(u32) -> i32} =
            core::mem::transmute(GetProcAddress(kernel32, b"SetConsoleOutputCP\0".as_ptr()));
        SetConsoleOutputCP(WinApi::CP_UTF8); // for stdout

        allocator::install_malloc_impl(
            dlmalloc_alloc,
            dlmalloc_alloc_zeroed,
            dlmalloc_dealloc,
            dlmalloc_realloc,
        );
        services::install_single_service(5, services_override::svc_read_stdio as usize);
        services::install_single_service(6, services_override::svc_write_stdio as usize);
    }
}
