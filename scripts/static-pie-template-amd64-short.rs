// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!
// SOLUTION BEGIN
#![crate_type="cdylib"]#![no_std]#[cfg(any())]mod x{
$$$$solution_src$$$$
}
// SOLUTION END
#[no_link]extern crate std;static mut P:[u8;$$$$binary_base91_len$$$$]=*br$$$$binary_base91$$$$;#[no_mangle]unsafe fn _start(){std::arch::asm!("stc;.quad 19510173000030c8h,4ce8d9f7c9h,459927e36758096ah,870d74ff8548c931h,4100003000b841cah,0b2ce8956e7ff40b1h,41ff6a5a41226a07h,0c11fb0c35e050f58h,99f572242cac0de0h,15bc06b242cac92h,10c4f608e8c1aad0h,5052535be3ebf775h,20ec834851c1ff51h,0c93197485750d3ffh,90c9d0ff585fd3ffh",in("r14")P.as_mut_ptr(),in("rsi")r$$$$stub_base91$$$$.as_ptr())}