// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!
// SOLUTION BEGIN
#![crate_type="cdylib"]#![no_std]#[cfg(any())]mod x{
$$$$solution_src$$$$
}
// SOLUTION END
#[no_link]extern crate std;static mut P:[u8;$$$$binary_base91_len$$$$]=*br$$$$binary_base91$$$$;#[no_mangle]unsafe fn _start(){std::arch::asm!(".quad 0e859016a000038c8h,6758096a0000003ch,3156c931459917e3h,41226a07b2ce89ffh,5e050f5841ff6a5ah,2cac0de0c11fb0c3h,242cac9299f57224h,0e8c1aad0015bc06bh,0e3ebf77510c4f608h,51c1ff515052535bh,4850d3ff28ec8348h,0e3ffc93197h",in("r14")P.as_mut_ptr(),in("rsi")r$$$$stub_base91$$$$.as_ptr())}