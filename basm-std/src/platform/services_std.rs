use std::io::{Read, Write};
use std::{io, process};

#[inline(always)]
pub fn exit(status: i32) -> ! {
    process::exit(status)
}
#[inline(always)]
pub fn read_stdio(fd: usize, buf: &mut [u8]) -> usize {
    match fd {
        0 => {
            let mut stdin = io::stdin();
            if let Ok(bytes_transferred) = stdin.read(buf) {
                bytes_transferred
            } else {
                0
            }
        }
        _ => 0,
    }
}
#[inline(always)]
pub fn write_stdio(fd: usize, buf: &[u8]) -> usize {
    match fd {
        1 => {
            let mut stdout = io::stdout();
            if let Ok(bytes_transferred) = stdout.write(buf) {
                bytes_transferred
            } else {
                0
            }
        }
        2 => {
            let mut stderr = io::stderr();
            if let Ok(bytes_transferred) = stderr.write(buf) {
                bytes_transferred
            } else {
                0
            }
        }
        _ => 0,
    }
}
