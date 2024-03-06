#![allow(clippy::not_unsafe_ptr_arg_deref)]

use core::ptr;
use core::arch::wasm32;
use super::dlmalloc_interface::DlmallocAllocator;


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
        let pages = (size + 65535) >> 16;
        let addr = wasm32::memory_grow(0, pages);
        if addr == usize::MAX {
            (ptr::null_mut(), 0, 0)
        } else {
            ((addr << 16) as *mut u8, pages << 16, 0)
        }
    }

    #[allow(unused)]
    fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8 {
        core::ptr::null_mut()
    }

    #[allow(unused)]
    fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
        false
    }

    #[allow(unused)]
    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        /* wasm32 does not support freeing memory */
        true
    }

    #[allow(unused)]
    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        /* wasm32 zeros memory upon grow.
         * see: https://webassembly.github.io/spec/core/exec/modules.html#grow-mem
         */
        true
    }

    fn page_size(&self) -> usize {
        65536
    }
}