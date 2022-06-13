use super::Stat;

pub fn read(_fd: u32, _s: &mut [u8]) -> isize {
    todo!();
}

pub fn write(_fd: u32, _s: &[u8]) -> isize {
    todo!()
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
