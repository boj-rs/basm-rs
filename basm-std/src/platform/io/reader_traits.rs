use alloc::string::String;
use super::{ReaderTrait, Readable};

impl Readable for String {
    fn read(reader: &mut impl ReaderTrait) -> Self {
        reader.word()
    }
}

#[allow(dead_code)]
pub struct Line(pub String);

impl Readable for Line {
    fn read(reader: &mut impl ReaderTrait) -> Self {
        Self(reader.line())
    }
}