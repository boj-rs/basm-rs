/// Special type that maps to `std::pair<T1, T2>`
/// on function implementation problems.
/// 
/// To create a new instance, use like `Pair(-3i8, 7u64)`.
/// 
/// Note: `(T1, T2)` maps to `std::tuple<T1, T2>`.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Pair<T1, T2>(T1, T2);

mod serialize;
pub use serialize::Ser as Serialize;
mod deserialize;
pub use deserialize::De as Deserialize;

pub unsafe fn eat(ptr_serialized: usize) -> (&'static [u8], usize) {
    const SIZE: usize = core::mem::size_of::<usize>();
    let mut buf = core::slice::from_raw_parts(ptr_serialized as *const u8, SIZE);
    let len = usize::de(&mut buf);
    let remain = core::slice::from_raw_parts((ptr_serialized + SIZE) as *const u8, len + SIZE);
    let mut buf = &remain[len..];
    let ptr_fn = usize::de(&mut buf);
    (&remain[..len], ptr_fn)
}