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