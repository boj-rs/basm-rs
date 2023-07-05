use basm::io::{Reader, Writer};

#[inline(always)]
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.next_usize();
    let b = reader.next_usize();
    writer.write_usize(a + b);
}
