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