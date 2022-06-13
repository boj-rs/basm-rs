#[cfg(test)]
pub use dummy::*;
#[cfg(not(test))]
pub use real::*;

#[cfg(test)]
pub mod dummy;
#[cfg(not(test))]
pub mod real;

#[repr(C)]
pub struct Stat {
    pub dev: u64,
    pub ino: u64,
    pub nlink: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    _pad: i32,
    pub rdev: u64,
    pub size: i64,
    pub blksize: i64,
    pub blocks: i64,
    pub atime: i64,
    pub atime_nsec: i64,
    pub mtime: i64,
    pub mtime_nsec: i64,
    pub ctime: i64,
    pub ctime_nsec: i64,
    _unused: [i64; 3],
}
