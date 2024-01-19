use basm::platform::io::{Reader, ReaderTrait, Writer, Print};
use core::cmp::max;
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let mut x = [usize::MAX; 100_001]; // x[i] = minimum end-value of "len >= i" increasing seq.
    while !reader.is_eof_skip_whitespace() {
        let n = reader.usize();
        let mut ans = 0;
        x[0] = 0;
        for i in 0..n {
            x[i + 1] = usize::MAX;
            let v = reader.usize();
            let (mut lo, mut hi) = (0, i);
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if x[mid] < v { lo = mid; } else { hi = mid - 1; }
            }
            let ans_new = lo + 1;
            x[ans_new] = v;
            ans = max(ans, ans_new);
        }
        writer.println(ans);
    }
}