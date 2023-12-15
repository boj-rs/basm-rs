use basm::platform::io::{Reader, Writer, Print};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.i64();
    let b = reader.i64();
    let p = [a as i32, b as i32];
    writer.println(sum(p.as_ptr(), 2));
}

use basm_macro::basm_export;
#[basm_export]
fn sum(a: *const i32, b: i32) -> i64 {
    unsafe {
        let mut out = 0;
        for i in 0..b as usize {
            out += *a.add(i) as i64;
        }
        out
    }
}