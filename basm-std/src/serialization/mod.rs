/// Special type that maps to `std::pair<T1, T2>`
/// on function implementation problems.
///
/// To create a new instance, use like `Pair(-3i8, 7u64)`.
///
/// Note: `(T1, T2)` maps to `std::tuple<T1, T2>`.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Pair<T1, T2>(pub T1, pub T2);

mod serialize;
pub use serialize::Ser;
mod deserialize;
pub use deserialize::De;

pub unsafe fn eat(ptr_serialized: usize) -> &'static [u8] {
    const SIZE: usize = core::mem::size_of::<usize>();
    let mut buf = core::slice::from_raw_parts(ptr_serialized as *const u8, SIZE);
    let len = usize::de(&mut buf);
    core::slice::from_raw_parts((ptr_serialized + SIZE) as *const u8, len)
}

/// Calls external function `ptr_fn_remote: fn(usize) -> usize` on function implementation problems.
/// Intended to be internal use only.
pub unsafe fn call_import(ptr_fn_remote: usize, ptr_serialized: usize) -> usize {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        let fn_remote: extern "win64" fn(usize) -> usize = core::mem::transmute(ptr_fn_remote);
        #[cfg(not(target_arch = "x86_64"))]
        let fn_remote: extern "C" fn(usize) -> usize = core::mem::transmute(ptr_fn_remote);

        fn_remote(ptr_serialized)
    }
}

/// Calls external function `ptr_free_remote: fn() -> ()` on function implementation problems.
/// Intended to be internal use only.
pub unsafe fn call_free(ptr_free_remote: usize) {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        let free_remote: extern "win64" fn() -> () = core::mem::transmute(ptr_free_remote);
        #[cfg(not(target_arch = "x86_64"))]
        let free_remote: extern "C" fn() -> () = core::mem::transmute(ptr_free_remote);

        free_remote()
    }
}
