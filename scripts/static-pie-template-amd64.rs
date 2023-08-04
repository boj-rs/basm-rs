// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box ☆ https://doc.rust-lang.org/book/

//==============================================================================
// SOLUTION BEGIN
//==============================================================================
#[cfg(any())]
mod solution {
$$$$solution_src$$$$
}
//==============================================================================
// SOLUTION END
//==============================================================================

//==============================================================================
// LOADER BEGIN
//==============================================================================
// Code adapted from:
//     https://github.com/kkamagui/mint64os/blob/master/02.Kernel64/Source/Loader.c
//     https://github.com/rafagafe/base85/blob/master/base85.c
//==============================================================================

use std::alloc::{alloc, alloc_zeroed, dealloc, realloc, Layout};
use std::arch::asm;
use std::env;
use std::io::{Read, Write, stdin, stdout, stderr};
use std::ptr::null;

////////////////////////////////////////////////////////////////////////////////
//
// Base85 decoder
//
////////////////////////////////////////////////////////////////////////////////

unsafe fn b85tobin(dest: *mut u8, mut src: *const u8) {
    let b85 = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";
    let mut p: *mut u32 = unsafe { core::mem::transmute(dest) };
    let mut digittobin: [u8; 256] = [0; 256];
    for i in 0..85 { digittobin[b85[i] as usize] = i as u8; }
    loop {
        while *src == b'\0' { src = src.wrapping_add(1); }
        if *src == b']' { break; }
        let mut value: u32 = 0;
        for _i in 0..5 {
            value *= 85;
            value += digittobin[*src as usize] as u32;
            src = src.wrapping_add(1);
        }
        *p = (value >> 24) | ((value >> 8) & 0xff00) | ((value << 8) & 0xff0000) | (value << 24);
        p = p.wrapping_add(1);
    }
}

////////////////////////////////////////////////////////////////////////////////
//
// Service functions
//
////////////////////////////////////////////////////////////////////////////////


#[repr(packed)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct PlatformData {
    env_id:                 u64,
    env_flags:              u64,
    pe_image_base:          u64,
    pe_off_reloc:           u64,
    pe_size_reloc:          u64,
    win_GetModuleHandleW:   u64,    // pointer to kernel32::GetModuleHandleW
    win_GetProcAddress:     u64,    // pointer to kernel32::GetProcAddress
}

type NativeFuncA = unsafe extern "win64" fn(usize, usize) -> *mut u8;
type NativeFuncB = unsafe extern "win64" fn(*mut u8, usize, usize);
type NativeFuncC = unsafe extern "win64" fn(*mut u8, usize, usize, usize) -> *mut u8;
type NativeFuncD = unsafe extern "win64" fn(usize) -> !;
type NativeFuncE = unsafe extern "win64" fn(usize, *mut u8, usize) -> usize;
type NativeFuncF = unsafe extern "win64" fn(usize, *const u8, usize) -> usize;
type NativeFuncG = unsafe extern "win64" fn(usize) -> *mut u8;

#[repr(packed)]
#[allow(dead_code)]
struct ServiceFunctions {
    ptr_imagebase:      usize,
    ptr_alloc:          NativeFuncA,
    ptr_alloc_zeroed:   NativeFuncA,
    ptr_dealloc:        NativeFuncB,
    ptr_realloc:        NativeFuncC,
    ptr_exit:           NativeFuncD,
    ptr_read_stdio:     NativeFuncE,
    ptr_write_stdio:    NativeFuncF,
    ptr_alloc_rwx:      NativeFuncG,
    ptr_platform:       usize,
}

unsafe extern "win64" fn svc_alloc(size: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).unwrap();
    alloc(layout)
}
unsafe extern "win64" fn svc_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).unwrap();
    alloc_zeroed(layout)
}
unsafe extern "win64" fn svc_free(ptr: *mut u8, size: usize, align: usize) {
    let layout = Layout::from_size_align(size, align).unwrap();
    dealloc(ptr, layout)
}
unsafe extern "win64" fn svc_realloc(memblock: *mut u8, old_size: usize, old_align: usize, new_size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(old_size, old_align).unwrap();
    realloc(memblock, layout, new_size)
}
unsafe extern "win64" fn svc_exit(status: usize) -> ! {
    std::process::exit(status as i32)
}
unsafe extern "win64" fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
    let slice = std::slice::from_raw_parts_mut(buf, count);
    match fd {
        0 => match stdin().read(slice) {
            Ok(x) => x,
            _error => 0,
        },
        _ => { 0 },
    }
}
unsafe extern "win64" fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
    let slice = std::slice::from_raw_parts(buf, count);
    match fd {
        1 => match stdout().write(slice) {
            Ok(x) => x,
            _error => 0,
        },
        2 => match stderr().write(slice) {
            Ok(x) => x,
            _error => 0,
        },
        _ => { 0 },
    }
}
#[inline(always)]
pub fn mmap(
    addr: *const u8,
    len: usize,
    protect: i32,
    flags: i32,
    fd: u32,
    offset: isize,
) -> *mut u8 {
    let out;
    unsafe {
        asm!("syscall", in("rax") 9, in("rdi") addr, in("rsi") len, in("rdx") protect, in("r10") flags, in("r8") fd, in("r9") offset, lateout("rax") out, out("rcx") _, out("r11") _);
    }
    out
}
static mut G_DEBUG: u32 = 0;
static mut RUN_COUNT: usize = 0;
unsafe extern "win64" fn svc_alloc_rwx(size: usize) -> *mut u8 {
    // currently Linux-only
    if RUN_COUNT == 1 && G_DEBUG != 0 {
        RUN_COUNT += 1;
        mmap(0x2000_0000usize as *const u8, size, 0x7, 0x32, 0xffffffff, 0)
    } else {
        if RUN_COUNT < 2 { RUN_COUNT += 1; }
        mmap(null(), size, 0x7, 0x22, 0xffffffff, 0)
    }
}

type StubPtr = unsafe extern "win64" fn(*mut u8, *const u8, usize, usize) -> !;

const STUB_BASE85: &[u8] = b$$$$stub_base85$$$$;
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;
const ENTRYPOINT_OFFSET: usize = $$$$entrypoint_offset$$$$;

fn main() {
    unsafe {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 2 && args[1] == "--debug" {
            G_DEBUG = 1;
        }
        let mut pd = PlatformData {
            env_id:                 0,      // For Rust, we default to ENV_ID_UNKNOWN
            env_flags:              if cfg!(windows) { 0 } else { 1 },
            pe_image_base:          $$$$pe_image_base$$$$u64,
            pe_off_reloc:           $$$$pe_off_reloc$$$$u64,
            pe_size_reloc:          $$$$pe_size_reloc$$$$u64,
            win_GetModuleHandleW:   0,      // [TBD] pointer to kernel32::GetModuleHandleW
            win_GetProcAddress:     0,      // [TBD] pointer to kernel32::GetProcAddress
        };
        let mut sf = ServiceFunctions {
            ptr_imagebase:      0,
            ptr_alloc:          svc_alloc,
            ptr_alloc_zeroed:   svc_alloc_zeroed,
            ptr_dealloc:        svc_free,
            ptr_realloc:        svc_realloc,
            ptr_exit:           svc_exit,
            ptr_read_stdio:     svc_read_stdio,
            ptr_write_stdio:    svc_write_stdio,
            ptr_alloc_rwx:      svc_alloc_rwx,
            ptr_platform:       std::ptr::addr_of_mut!(pd) as usize,
        };

        let stub = svc_alloc_rwx(0x1000);
        b85tobin(stub, STUB_BASE85.as_ptr());
        b85tobin(BINARY_BASE85.as_mut_ptr(), BINARY_BASE85.as_ptr());
        let stub_fn: StubPtr = core::mem::transmute(stub);
        stub_fn(core::mem::transmute(std::ptr::addr_of_mut!(sf)), BINARY_BASE85.as_ptr(), ENTRYPOINT_OFFSET, G_DEBUG as usize);
    }
}
//==============================================================================
// LOADER END
//==============================================================================