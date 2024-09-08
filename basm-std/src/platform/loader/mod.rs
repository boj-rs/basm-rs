#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub mod aarch64_elf;
#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
pub mod amd64_elf;
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub mod amd64_pe;
#[cfg(all(target_arch = "x86", not(target_os = "windows")))]
pub mod i686_elf;
