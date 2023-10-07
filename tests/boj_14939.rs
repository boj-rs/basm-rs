use basm::platform::io::{Reader, Writer, Print};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let mut board = [0; 10];
    let n = 10;
    for i in 0..n {
        for j in 0..n {
            reader.skip_whitespace();
            let c = reader.ascii();
            if c == b'O' {
                board[i] |= 1u32 << j;
            }
        }
    }
    let mut ans = u32::MAX;
    for b in 0..1u32<<n {
        let mut cnt = 0;
        let mut state = b;
        let mut cmd = 0;
        for i in 0..n {
            (state, cmd) = ({
                let mut out = board[i] ^ cmd;
                for j in 0..n {
                    if state & (1u32 << j) != 0 {
                        if j > 0 { out ^= 1u32 << (j - 1); }
                        out ^= 1u32 << j;
                        if j + 1 < n { out ^= 1u32 << (j + 1); }
                    }
                }
                out
            }, state);
            cnt += cmd.count_ones();
        }
        if state == 0 { ans = cnt; }
    }
    writer.println(if ans == u32::MAX { -1 } else { ans as i32 });
}