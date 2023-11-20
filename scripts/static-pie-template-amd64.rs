// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, omit this line or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as s;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(all(target_arch = "x86_64", any(windows, target_os = "linux"))))]
compile_error!("Unsupported target architecture or operating system.");
#[cfg(windows)]
macro_rules! p { () => { "lea rcx,[rip+78];call LoadLibraryA;lea rdx,[rip+GetProcAddress];lea rdi,[rip+VirtualAlloc];clc" } }
#[cfg(not(windows))]
macro_rules! p { () => { "stc" } }

static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;
#[no_mangle]
unsafe fn _start() {
    s::arch::asm!(p!(),
        ".quad 19510173000030c8h,53e8d9f7c9h,48c931459958096ah,0b841ca870d74ff85h,\
        0ff40b14100003000h,226a07b2ce8956e7h,50f5841ff6a5a41h,6c656e72656bc35eh,\
        0e0c1581f6a003233h,9299eb72242cac0dh,0d0015bc06b242cach,7510c4f608e8c1aah,\
        0c1ff515052e3ebf7h,2454ff20ec834851h,0ffc9e89748505740h,575f565e5641ffffh,\
        5f585affffffbee8h,0c9d0ff20244c8d48h", in("r14") PAYLOAD.as_mut_ptr(), in("rsi") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END