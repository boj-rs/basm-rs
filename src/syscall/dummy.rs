use alloc::collections::VecDeque;
use alloc::vec::Vec;
use std::lazy::OnceCell;

use super::Stat;

static mut STDIN: OnceCell<VecDeque<u8>> = OnceCell::new();
static mut STDOUT: OnceCell<Vec<u8>> = OnceCell::new();
static mut STDERR: OnceCell<Vec<u8>> = OnceCell::new();

/// Only for test
pub fn prepare_stdin(content: &[u8]) {
    let v = unsafe {
        STDIN.get_or_init(|| VecDeque::with_capacity(content.len()));
        STDIN.get_mut().unwrap()
    };
    v.clear();
    v.extend(content);
}

/// Only for test
pub fn get_stdout_content() -> &'static [u8] {
    unsafe { STDOUT.get_or_init(Vec::new) }
}

pub fn get_stderr_content() -> &'static [u8] {
    unsafe { STDOUT.get_or_init(Vec::new) }
}

pub fn read(fd: u32, s: &mut [u8]) -> isize {
    assert_eq!(fd, 0);
    let len;
    unsafe {
        let v = STDIN
            .get_mut()
            .expect("Prepare test stdin using prepare_stdin() before using read().");
        len = s.len().min(v.len());
        for (p, c) in s.iter_mut().zip(v.drain(..len)) {
            *p = c;
        }
    }
    len as isize
}

pub fn write(fd: u32, s: &[u8]) -> isize {
    assert!(fd == 1 || fd == 2);
    let stream = unsafe {
        if fd == 1 {
            &mut STDOUT
        } else {
            &mut STDERR
        }
    };
    stream.get_or_init(|| Vec::with_capacity(s.len()));
    stream.get_mut().unwrap().extend(s);
    s.len() as isize
}

pub fn mmap(
    _addr: *const u8,
    _len: usize,
    _protect: i32,
    _flags: i32,
    _fd: u32,
    _offset: isize,
) -> *mut u8 {
    todo!()
}

pub fn fstat(_fd: u32) -> Stat {
    todo!()
}
