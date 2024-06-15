mod reader;
pub use reader::{Readable, Reader, ReaderTrait};
mod writer;
pub use writer::{Print, Writer};
mod reader_traits;
pub use reader_traits::*;
const DEFAULT_BUF_SIZE: usize = 1 << 16;
#[allow(dead_code)]
const MIN_BUF_SIZE: usize = 128;
