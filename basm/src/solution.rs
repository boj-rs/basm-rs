pub fn main() {
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