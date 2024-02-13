pub mod dlmalloc_interface;
pub mod dlmalloc;
#[cfg(not(any(target_arch = "wasm32", target_arch = "aarch64")))]
pub mod dlmalloc_windows;
#[cfg(not(any(target_arch = "wasm32", target_arch = "aarch64")))]
pub mod dlmalloc_linux;
#[cfg(target_arch = "aarch64")]
pub mod dlmalloc_macos;
#[cfg(target_arch = "wasm32")]
pub mod dlmalloc_wasm32;