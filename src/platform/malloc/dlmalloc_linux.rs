use super::dlmalloc_interface::DlmallocAllocator;


pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

mod linux_syscall {
    use core::arch::asm;
    pub const PROT_READ: i32 = 0x01;
    pub const PROT_WRITE: i32 = 0x02;
    pub const MAP_PRIVATE: i32 = 0x02;
    pub const MAP_ANON: i32 = 0x20;
    pub const MREMAP_MAYMOVE: i32 = 0x01;
    pub const MAP_FAILED: *mut u8 = unsafe { core::mem::transmute(usize::MAX) };

    #[cfg(target_arch = "x86_64")]
    mod id_list {
        pub const MMAP: usize = 9;
        pub const MREMAP: usize = 25;
        pub const MUNMAP: usize = 11;
    }
    #[cfg(target_arch = "x86")]
    mod id_list {
        pub const MMAP: usize = 90;
        pub const MREMAP: usize = 163;
        pub const MUNMAP: usize = 91;
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    unsafe fn syscall(
        call_id: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize
    ) -> usize {
        let out;
        asm!(
            "syscall",
            in("rax") call_id,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            in("r8") arg4,
            in("r9") arg5,
            lateout("rax") out,
            out("rcx") _,
            out("r11") _
        );
        out
    }
    #[cfg(target_arch = "x86")]
    #[naked]
    unsafe extern "cdecl" fn syscall(
        call_id: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize
    ) -> usize {
        asm!(
            "push ebp",
            "push ebx",
            "push esi",
            "push edi",
            "mov eax, DWORD PTR [esp + 20]",
            "mov ebx, DWORD PTR [esp + 24]",
            "mov ecx, DWORD PTR [esp + 28]",
            "mov edx, DWORD PTR [esp + 32]",
            "mov esi, DWORD PTR [esp + 36]",
            "mov edi, DWORD PTR [esp + 40]",
            "mov ebp, DWORD PTR [esp + 44]",
            "int 0x80",
            "pop edi",
            "pop esi",
            "pop ebx",
            "pop ebp",
            "ret",
            options(noreturn)
        );
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub unsafe fn mmap(
        addr: *const u8,
        len: usize,
        protect: i32,
        flags: i32,
        fd: i32,
        offset: isize,
    ) -> *mut u8 {
        syscall(id_list::MMAP, addr as usize, len, protect as usize, flags as usize, fd as usize, offset as usize) as *mut u8
    }
    #[cfg(target_arch = "x86")]
    #[inline(always)]
    pub unsafe fn mmap(
        addr: *const u8,
        len: usize,
        protect: i32,
        flags: i32,
        fd: i32,
        offset: isize,
    ) -> *mut u8 {
        let mmap_arg_struct = [addr as usize, len, protect as usize, flags as usize, fd as usize, offset as usize];
        syscall(id_list::MMAP, mmap_arg_struct.as_ptr() as usize, 0, 0, 0, 0, 0) as *mut u8
    }
    #[inline(always)]
    pub unsafe fn mremap(
        old_address: *const u8,
        old_size: usize,
        new_size: usize,
        flags: i32,
    ) -> *mut u8 {
        syscall(id_list::MREMAP, old_address as usize, old_size, new_size, flags as usize, 0, 0) as *mut u8
    }
    #[inline(always)]
    pub unsafe fn munmap(
        addr: *const u8,
        len: usize,
    ) -> *mut u8 {
        syscall(id_list::MUNMAP, addr as usize, len, 0, 0, 0, 0) as *mut u8
    }
}

unsafe impl DlmallocAllocator for System {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let addr = unsafe {
            linux_syscall::mmap(
                0 as *mut _,
                size,
                linux_syscall::PROT_WRITE | linux_syscall::PROT_READ,
                linux_syscall::MAP_ANON | linux_syscall::MAP_PRIVATE,
                -1,
                0,
            )
        };
        if core::ptr::eq(addr, linux_syscall::MAP_FAILED) {
            (core::ptr::null_mut(), 0, 0)
        } else {
            (addr as *mut u8, size, 0)
        }
    }

    fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8 {
        let flags = if can_move { linux_syscall::MREMAP_MAYMOVE } else { 0 };
        let ptr = unsafe { linux_syscall::mremap(ptr as *mut _, oldsize, newsize, flags) };
        if core::ptr::eq(ptr, linux_syscall::MAP_FAILED) {
            core::ptr::null_mut()
        } else {
            ptr as *mut u8
        }
    }

    fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
        unsafe {
            let rc = linux_syscall::mremap(ptr as *mut _, oldsize, newsize, 0);
            if !core::ptr::eq(rc, linux_syscall::MAP_FAILED) {
                return true;
            }
            linux_syscall::munmap(ptr.offset(newsize as isize) as *mut _, oldsize - newsize).is_null()
        }
    }

    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        unsafe { linux_syscall::munmap(ptr as *mut _, size).is_null() }
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