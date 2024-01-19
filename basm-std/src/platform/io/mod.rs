mod reader;
pub use reader::{Reader, ReaderTrait, Readable};
mod writer;
pub use writer::{Writer, Print};
mod reader_traits;
pub use reader_traits::*;
const DEFAULT_BUF_SIZE: usize = 1 << 16;
#[allow(dead_code)]
const MIN_BUF_SIZE: usize = 128;