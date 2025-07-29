use basm::platform::io::*;
use alloc::collections::BTreeSet;
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let mut s = BTreeSet::new();
    let n = reader.usize();
    for _ in 0..n {
        s.insert(reader.i32());
    }
    for x in s {
        writer.println(x);
    }
}