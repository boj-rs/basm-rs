#![allow(clippy::not_unsafe_ptr_arg_deref)]

use super::dlmalloc_interface::DlmallocAllocator;
use super::super::os::linux::syscall;


pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl DlmallocAllocator for System {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let addr = unsafe {
            syscall::mmap(
                core::ptr::null_mut(),
                size,
                syscall::PROT_WRITE | syscall::PROT_READ,
                syscall::MAP_ANON | syscall::MAP_PRIVATE,
                -1,
                0,
            )
        };
        if core::ptr::eq(addr, syscall::MAP_FAILED) {
            (core::ptr::null_mut(), 0, 0)
        } else {
            (addr, size, 0)
        }
    }

    fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8 {
        let flags = if can_move { syscall::MREMAP_MAYMOVE } else { 0 };
        let ptr = unsafe { syscall::mremap(ptr as *mut _, oldsize, newsize, flags) };
        if core::ptr::eq(ptr, syscall::MAP_FAILED) {
            core::ptr::null_mut()
        } else {
            ptr
        }
    }

    fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
        unsafe {
            let rc = syscall::mremap(ptr as *mut _, oldsize, newsize, 0);
            if !core::ptr::eq(rc, syscall::MAP_FAILED) {
                return true;
            }
            syscall::munmap(ptr.add(newsize) as *mut _, oldsize - newsize).is_null()
        }
    }

    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        unsafe { syscall::munmap(ptr as *mut _, size).is_null() }
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        true
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        4096
    }
}