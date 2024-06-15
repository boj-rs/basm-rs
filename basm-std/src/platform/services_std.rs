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
            stdin.read(buf).unwrap_or_default()
        }
        _ => 0,
    }
}
#[inline(always)]
pub fn write_stdio(fd: usize, buf: &[u8]) -> usize {
    match fd {
        1 => {
            let mut stdout = io::stdout();
            stdout.write(buf).unwrap_or_default()
        }
        2 => {
            let mut stderr = io::stderr();
            stderr.write(buf).unwrap_or_default()
        }
        _ => 0,
    }
}
