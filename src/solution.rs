use basm::io::{Reader, Writer};

#[cfg_attr(not(debug_assertions), inline(always))]
#[cfg_attr(debug_assertions, inline(never))]
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.next_usize();
    let b = reader.next_usize();
    writer.write_usize(a + b);
}
