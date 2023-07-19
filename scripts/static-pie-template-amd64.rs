// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box ☆ https://doc.rust-lang.org/book/

//==============================================================================
// SOLUTION BEGIN
//==============================================================================
$$$$solution_src$$$$
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
use std::mem::size_of;
use std::ptr::{null, null_mut};

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


type NativeFuncA = unsafe extern "C" fn(usize) -> *mut u8;
type NativeFuncB = unsafe extern "C" fn(*mut u8);
type NativeFuncC = unsafe extern "C" fn(*mut u8, usize) -> *mut u8;
type NativeFuncD = unsafe extern "C" fn(usize) -> !;
type NativeFuncE = unsafe extern "C" fn(usize, *mut u8, usize) -> usize;
type NativeFuncF = unsafe extern "C" fn(usize, *const u8, usize) -> usize;

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
    ptr_alloc_rwx:      NativeFuncA,
}

unsafe extern "C" fn svc_alloc(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(size_of::<usize>() + size).unwrap();
    let mut ptr = alloc(layout);
    if ptr != null_mut() {
        let ptr_size: *mut usize = core::mem::transmute(ptr);
        *ptr_size = size;
        ptr = ptr.wrapping_add(size_of::<usize>());
    }
    ptr
}
unsafe extern "C" fn svc_alloc_zeroed(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(size_of::<usize>() + size).unwrap();
    let mut ptr = alloc_zeroed(layout);
    if ptr != null_mut() {
        let ptr_size: *mut usize = core::mem::transmute(ptr);
        *ptr_size = size;
        ptr = ptr.wrapping_add(size_of::<usize>());
    }
    ptr
}
unsafe extern "C" fn svc_free(ptr: *mut u8) {
    let ptr_orig = ptr.wrapping_sub(size_of::<usize>());
    let ptr_size: *mut usize = core::mem::transmute(ptr_orig);
    let size_orig = *ptr_size;
    let layout = Layout::array::<u8>(size_of::<usize>() + size_orig).unwrap();
    dealloc(ptr_orig, layout);
}
unsafe extern "C" fn svc_realloc(memblock: *mut u8, size: usize) -> *mut u8 {
    let ptr_orig = memblock.wrapping_sub(size_of::<usize>());
    let ptr_size: *mut usize = core::mem::transmute(ptr_orig);
    let size_orig = *ptr_size;
    let layout = Layout::array::<u8>(size_of::<usize>() + size_orig).unwrap();
    let mut ptr = realloc(ptr_orig, layout, size);
    if ptr != null_mut() {
        let ptr_size: *mut usize = core::mem::transmute(ptr);
        *ptr_size = size;
        ptr = ptr.wrapping_add(size_of::<usize>());
    }
    ptr
}
unsafe extern "C" fn svc_exit(status: usize) -> ! {
    std::process::exit(status as i32)
}
unsafe extern "C" fn svc_read_stdio(fd: usize, buf: *mut u8, count: usize) -> usize {
    let slice = std::slice::from_raw_parts_mut(buf, count);
    match fd {
        0 => match stdin().read(slice) {
            Ok(x) => x,
            _error => 0,
        },
        _ => { 0 },
    }
}
unsafe extern "C" fn svc_write_stdio(fd: usize, buf: *const u8, count: usize) -> usize {
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
unsafe extern "C" fn svc_alloc_rwx(size: usize) -> *mut u8 {
    // currently Linux-only
    if RUN_COUNT == 1 && G_DEBUG != 0 {
        RUN_COUNT += 1;
        mmap(0x2000_0000usize as *const u8, size, 0x7, 0x32, 0xffffffff, 0)
    } else {
        if RUN_COUNT < 2 { RUN_COUNT += 1; }
        mmap(null(), size, 0x7, 0x22, 0xffffffff, 0)
    }
}

type StubPtr = unsafe extern "C" fn(*mut u8, *const u8, usize, usize) -> !;

const STUB_BASE85: &[u8] = b$$$$stub_base85$$$$;
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;
const ENTRYPOINT_OFFSET: usize = $$$$entrypoint_offset$$$$;

fn main() {
    unsafe {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 2 && args[1] == "--debug" {
            G_DEBUG = 1;
        }
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