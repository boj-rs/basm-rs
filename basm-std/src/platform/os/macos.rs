use super::super::{allocator, services};
use super::super::malloc::{dlmalloc, dlmalloc_macos};


pub mod syscall {
    pub const PROT_READ: i32 = 0x01;
    pub const PROT_WRITE: i32 = 0x02;
    pub const MAP_PRIVATE: i32 = 0x02;
    pub const MAP_ANON: i32 = 0x1000;
    pub const MREMAP_MAYMOVE: i32 = 0x01;
    pub const MAP_FAILED: *mut u8 = usize::MAX as *mut u8;
    pub const RLIMIT_STACK: usize = 3;

    #[derive(Default)]
    #[repr(C, packed)]
    pub struct RLimit {
        pub rlim_cur: usize,
        pub rlim_max: usize,
    }

    #[link(name = "System", kind = "dylib")]
    extern "C" {
        pub fn mmap(
            addr: *const u8,
            len: usize,
            protect: i32,
            flags: i32,
            fd: i32,
            offset: isize,
        ) -> *mut u8;
        pub fn munmap(
            addr: *const u8,
            len: usize,
        ) -> *mut u8;
        pub fn read(
            fd: usize,
            buf: *mut u8,
            count: usize
        ) -> usize;
        pub fn write(
            fd: usize,
            buf: *const u8,
            count: usize
        ) -> usize;
        #[link_name = "exit"]
        pub fn exit_group(
            status: usize
        ) -> !;
        pub fn getrlimit(
            resource: usize,
            rlim: *mut RLimit
        ) -> usize;
        pub fn setrlimit(
            resource: usize,
            rlim: *const RLimit
        ) -> usize;
    }
}

static mut DLMALLOC: dlmalloc::Dlmalloc<dlmalloc_macos::System> = dlmalloc::Dlmalloc::new(dlmalloc_macos::System::new());
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

mod services_override {
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
    /* Ensure stack size is at least 256 MiB, when running locally
     * (online judges usually have their stack sizes set large).
     * For Windows, this is set as a linker option in the build script.
     * However, on Linux, the linker option only marks this value
     * in an ELF section, which must be interpreted and applied
     * by the runtime startup code (e.g., glibc).
     * Thus, instead of parsing the ELF section, we just invoke
     * the kernel APIs directly. */
    let pd = services::platform_data();
    if pd.env_flags & services::ENV_FLAGS_NATIVE != 0 {
        let mut rlim: syscall::RLimit = Default::default();
        let ret = syscall::getrlimit(syscall::RLIMIT_STACK, &mut rlim as *mut syscall::RLimit);
        if ret == 0 && rlim.rlim_cur < 256 * 1024 * 1024 {
            rlim.rlim_cur = 256 * 1024 * 1024;
            syscall::setrlimit(syscall::RLIMIT_STACK, &rlim as *const syscall::RLimit);
        }
    }

    allocator::install_malloc_impl(
        dlmalloc_alloc,
        dlmalloc_alloc_zeroed,
        dlmalloc_dealloc,
        dlmalloc_realloc,
    );

    services::install_single_service(5, services_override::svc_read_stdio as usize);
    services::install_single_service(6, services_override::svc_write_stdio as usize);
}