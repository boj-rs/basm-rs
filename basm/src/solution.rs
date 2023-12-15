use basm::platform::io::{Reader, Writer, Print};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.i64();
    let b = reader.i64();
    writer.println(unsafe { _basm_export_sum_i64_i64_to_i64(a, b) });
}

use basm_macro::basm_export;
#[basm_export]
fn sum(a: i64, b: i64) -> i64 {
    a + b
}