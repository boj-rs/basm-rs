use super::super::allocator;
use super::super::malloc::{dlmalloc, dlmalloc_linux};

pub mod syscall {
    use core::arch::asm;
    pub const PROT_READ: i32 = 0x01;
    pub const PROT_WRITE: i32 = 0x02;
    pub const MAP_PRIVATE: i32 = 0x02;
    pub const MAP_ANON: i32 = 0x20;
    pub const MREMAP_MAYMOVE: i32 = 0x01;
    pub const MAP_FAILED: *mut u8 = usize::MAX as *mut u8;
    pub const RLIMIT_STACK: usize = 3;

    #[cfg(target_arch = "x86_64")]
    mod id_list {
        pub const READ: usize = 0;
        pub const WRITE: usize = 1;
        pub const MMAP: usize = 9;
        pub const MREMAP: usize = 25;
        pub const MUNMAP: usize = 11;
        pub const EXIT_GROUP: usize = 231;
        pub const GETRLIMIT: usize = 97;
        pub const SETRLIMIT: usize = 160;
    }
    #[cfg(target_arch = "x86")]
    mod id_list {
        pub const READ: usize = 3;
        pub const WRITE: usize = 4;
        pub const MMAP: usize = 90;
        pub const MREMAP: usize = 163;
        pub const MUNMAP: usize = 91;
        pub const EXIT_GROUP: usize = 252;
        pub const GETRLIMIT: usize = 76;
        pub const SETRLIMIT: usize = 75;
    }
    #[cfg(target_arch = "aarch64")]
    mod id_list {
        pub const READ: usize = 63;
        pub const WRITE: usize = 64;
        pub const MMAP: usize = 222;
        pub const MREMAP: usize = 216;
        pub const MUNMAP: usize = 215;
        pub const EXIT_GROUP: usize = 94;
        pub const GETRLIMIT: usize = 163;
        pub const SETRLIMIT: usize = 164;
    }

    #[derive(Default)]
    #[repr(packed)]
    pub struct RLimit {
        pub rlim_cur: usize,
        pub rlim_max: usize,
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub unsafe fn syscall1(call_id: usize, arg0: usize) -> usize {
        unsafe {
            let out;
            asm!(
                "syscall",
                in("rax") call_id,
                in("rdi") arg0,
                lateout("rax") out,
                out("rcx") _,
                out("r11") _
            );
            out
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    pub unsafe fn syscall1(call_id: usize, arg0: usize) -> usize {
        unsafe { syscall(call_id, arg0, 0, 0, 0, 0, 0) }
    }
    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub unsafe fn syscall3(call_id: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
        unsafe {
            let out;
            asm!(
                "syscall",
                in("rax") call_id,
                in("rdi") arg0,
                in("rsi") arg1,
                in("rdx") arg2,
                lateout("rax") out,
                out("rcx") _,
                out("r11") _
            );
            out
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    unsafe extern "cdecl" fn syscall3(
        call_id: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
    ) -> usize {
        unsafe { syscall(call_id, arg0, arg1, arg2, 0, 0, 0) }
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
        arg5: usize,
    ) -> usize {
        unsafe {
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
        arg5: usize,
    ) -> usize {
        unsafe {
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
    }
    #[cfg(target_arch = "aarch64")]
    pub unsafe fn syscall(
        call_id: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize {
        unsafe {
            let out;
            asm!(
                "svc #0",
                in("x8") call_id,
                in("x0") arg0,
                in("x1") arg1,
                in("x2") arg2,
                in("x3") arg3,
                in("x4") arg4,
                in("x5") arg5,
                lateout("x0") out
            );
            out
        }
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
        unsafe {
            syscall(
                id_list::MMAP,
                addr as usize,
                len,
                protect as usize,
                flags as usize,
                fd as usize,
                offset as usize,
            ) as *mut u8
        }
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
        unsafe {
            let mmap_arg_struct = [
                addr as usize,
                len,
                protect as usize,
                flags as usize,
                fd as usize,
                offset as usize,
            ];
            syscall(
                id_list::MMAP,
                mmap_arg_struct.as_ptr() as usize,
                0,
                0,
                0,
                0,
                0,
            ) as *mut u8
        }
    }
    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    pub unsafe fn mmap(
        addr: *const u8,
        len: usize,
        protect: i32,
        flags: i32,
        fd: i32,
        offset: isize,
    ) -> *mut u8 {
        unsafe {
            syscall(
                id_list::MMAP,
                addr as usize,
                len,
                protect as usize,
                flags as usize,
                fd as usize,
                offset as usize,
            ) as *mut u8
        }
    }
    #[inline(always)]
    pub unsafe fn mremap(
        old_address: *const u8,
        old_size: usize,
        new_size: usize,
        flags: i32,
    ) -> *mut u8 {
        unsafe {
            syscall(
                id_list::MREMAP,
                old_address as usize,
                old_size,
                new_size,
                flags as usize,
                0,
                0,
            ) as *mut u8
        }
    }
    #[inline(always)]
    pub unsafe fn munmap(addr: *const u8, len: usize) -> *mut u8 {
        unsafe { syscall(id_list::MUNMAP, addr as usize, len, 0, 0, 0, 0) as *mut u8 }
    }
    #[inline(always)]
    pub unsafe fn read(fd: usize, buf: *mut u8, count: usize) -> usize {
        unsafe { syscall3(id_list::READ, fd, buf as usize, count) }
    }
    #[inline(always)]
    pub unsafe fn write(fd: usize, buf: *const u8, count: usize) -> usize {
        unsafe { syscall3(id_list::WRITE, fd, buf as usize, count) }
    }
    #[inline(always)]
    pub unsafe fn exit_group(status: usize) -> ! {
        unsafe {
            syscall1(id_list::EXIT_GROUP, status);
            unreachable!()
        }
    }
    #[inline(always)]
    pub unsafe fn getrlimit(resource: usize, rlim: &mut RLimit) -> usize {
        unsafe {
            syscall(
                id_list::GETRLIMIT,
                resource,
                rlim as *mut RLimit as usize,
                0,
                0,
                0,
                0,
            )
        }
    }
    #[inline(always)]
    pub unsafe fn setrlimit(resource: usize, rlim: &RLimit) -> usize {
        unsafe {
            syscall(
                id_list::SETRLIMIT,
                resource,
                rlim as *const RLimit as usize,
                0,
                0,
                0,
                0,
            )
        }
    }
}

static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_linux::System> =
    dlmalloc::Dlmalloc::new(dlmalloc_linux::System::new());
unsafe fn dlmalloc_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { DLMALLOC.memalign(align, size) }
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

#[cfg(not(all(feature = "short", target_os = "linux")))]
#[cfg(target_arch = "x86_64")]
mod services_override {
    #[inline(always)]
    pub unsafe extern "win64" fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
        unsafe { super::syscall::read(fd, buf, count) }
    }
    #[inline(always)]
    pub unsafe extern "win64" fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
        unsafe { super::syscall::write(fd, buf, count) }
    }
}
#[cfg(not(all(feature = "short", target_os = "linux")))]
#[cfg(not(target_arch = "x86_64"))]
mod services_override {
    #[inline(always)]
    pub unsafe extern "C" fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
        unsafe { super::syscall::read(fd, buf, count) }
    }
    #[inline(always)]
    pub unsafe extern "C" fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
        unsafe { super::syscall::write(fd, buf, count) }
    }
}

pub unsafe fn init() {
    unsafe {
        /* Ensure stack size is at least 256 MiB, when running locally
         * (online judges usually have their stack sizes set large).
         * For Windows, this is set as a linker option in the build script.
         * However, on Linux, the linker option only marks this value
         * in an ELF section, which must be interpreted and applied
         * by the runtime startup code (e.g., glibc).
         * Thus, instead of parsing the ELF section, we just invoke
         * the kernel APIs directly. */
        #[cfg(not(feature = "short"))]
        {
            use super::super::services;
            let pd = services::platform_data();
            if pd.env_flags & services::ENV_FLAGS_NATIVE != 0 {
                let mut rlim: syscall::RLimit = Default::default();
                let ret = syscall::getrlimit(syscall::RLIMIT_STACK, &mut rlim);
                if ret == 0 && rlim.rlim_cur < 256 * 1024 * 1024 {
                    rlim.rlim_cur = 256 * 1024 * 1024;
                    syscall::setrlimit(syscall::RLIMIT_STACK, &rlim);
                }
            }
        }

        allocator::install_malloc_impl(
            dlmalloc_alloc,
            dlmalloc_alloc_zeroed,
            dlmalloc_dealloc,
            dlmalloc_realloc,
        );

        /* "short" on "Linux" will use syscalls directly to reduce code size */
        #[cfg(not(all(feature = "short", target_os = "linux")))]
        {
            use super::super::services;
            services::install_single_service(5, services_override::svc_read_stdio as usize);
            services::install_single_service(6, services_override::svc_write_stdio as usize);
        }
    }
}
