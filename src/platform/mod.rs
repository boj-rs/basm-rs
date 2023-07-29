#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
pub mod amd64;
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub mod amd64_windows;
#[cfg(target_arch = "x86")]
pub mod i686;