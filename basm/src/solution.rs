use basm::platform::io::*;
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.i64();
    let b = reader.i64();
    writer.println(a + b);
}
