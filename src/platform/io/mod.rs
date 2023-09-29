mod reader;
pub use reader::Reader;
mod writer;
pub use writer::{Writer, Print};
const DEFAULT_BUF_SIZE: usize = 1 << 16;
const MIN_BUF_SIZE: usize = 128;