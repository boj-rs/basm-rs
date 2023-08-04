use super::super::{allocator, services};
use super::super::malloc::{dlmalloc, dlmalloc_linux};


pub mod syscall {
    use core::arch::asm;
    pub const PROT_READ: i32 = 0x01;
    pub const PROT_WRITE: i32 = 0x02;
    pub const MAP_PRIVATE: i32 = 0x02;
    pub const MAP_ANON: i32 = 0x20;
    pub const MREMAP_MAYMOVE: i32 = 0x01;
    pub const MAP_FAILED: *mut u8 = unsafe { core::mem::transmute(usize::MAX) };

    #[cfg(target_arch = "x86_64")]
    mod id_list {
        pub const READ: usize = 0;
        pub const WRITE: usize = 1;
        pub const MMAP: usize = 9;
        pub const MREMAP: usize = 25;
        pub const MUNMAP: usize = 11;
        pub const EXIT_GROUP: usize = 231;
    }
    #[cfg(target_arch = "x86")]
    mod id_list {
        pub const READ: usize = 3;
        pub const WRITE: usize = 4;
        pub const MMAP: usize = 90;
        pub const MREMAP: usize = 163;
        pub const MUNMAP: usize = 91;
        pub const EXIT_GROUP: usize = 252;
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub unsafe fn syscall(
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
    #[inline(always)]
    pub unsafe fn read(
        fd: usize,
        buf: *mut u8,
        count: usize
    ) -> usize {
        syscall(id_list::READ, fd, buf as usize, count, 0, 0, 0)
    }
    #[inline(always)]
    pub unsafe fn write(
        fd: usize,
        buf: *const u8,
        count: usize
    ) -> usize {
        syscall(id_list::WRITE, fd, buf as usize, count, 0, 0, 0)
    }
    #[inline(always)]
    pub unsafe fn exit_group(
        status: usize
    ) -> ! {
        syscall(id_list::EXIT_GROUP, status, 0, 0, 0, 0, 0);
        unreachable!()
    }
}

static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_linux::System> = dlmalloc::Dlmalloc::new(dlmalloc_linux::System::new());
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

#[cfg(target_arch = "x86_64")]
mod services_override {
    #[inline(always)]
    pub unsafe extern "win64" fn svc_exit(status: usize) -> ! {
        super::syscall::exit_group(status)
    }
    #[inline(always)]
    pub unsafe extern "win64" fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
        super::syscall::read(fd, buf, count)
    }
    #[inline(always)]
    pub unsafe extern "win64" fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
        super::syscall::write(fd, buf, count)
    }
}
#[cfg(target_arch = "x86")]
mod services_override {
    #[inline(always)]
    pub unsafe extern "C" fn svc_exit(status: usize) -> ! {
        super::syscall::exit_group(status)
    }
    #[inline(always)]
    pub unsafe extern "C" fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
        super::syscall::read(fd, buf, count)
    }
    #[inline(always)]
    pub unsafe extern "C" fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
        super::syscall::write(fd, buf, count)
    }
}

pub unsafe fn init() {
    allocator::install_malloc_impl(
        dlmalloc_alloc,
        dlmalloc_alloc_zeroed,
        dlmalloc_dealloc,
        dlmalloc_realloc,
    );
    services::install_single_service(5, services_override::svc_exit as usize);
    services::install_single_service(6, services_override::svc_read_stdio as usize);
    services::install_single_service(7, services_override::svc_write_stdio as usize);
}