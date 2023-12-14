pub mod dlmalloc_interface;
pub mod dlmalloc;
#[cfg(not(target_arch = "wasm32"))]
pub mod dlmalloc_windows;
#[cfg(not(target_arch = "wasm32"))]
pub mod dlmalloc_linux;
#[cfg(target_arch = "wasm32")]
pub mod dlmalloc_wasm32;