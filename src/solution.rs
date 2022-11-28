use basm::syscall;

#[inline(always)]
pub fn main() {
    let mut input = [0u8; 1 << 16];
    let mut len = syscall::read(0, &mut input) as usize;
    let mut i = 0;
    while input[i] != b'\n' {
        i += 1;
    }
    i += 1;
    let mut c = input[i];
    i += 1;
    let mut streak = 1usize;
    let mut prev_streak = 0usize;
    let mut max = 0usize;
    while len > 0 {
        for j in i..len {
            if input[j] == c {
                streak += 1;
            } else {
                max = max.max(prev_streak.min(streak) * 2);
                prev_streak = streak;
                streak = 1;
                c = input[j];
            }
        }
        len = syscall::read(0, &mut input) as usize;
        i = 0;
    }
    i = input.len();
    loop {
        i -= 1;
        input[i] = (max % 10) as u8 + 48;
        max /= 10;
        if max == 0 {
            break;
        }
    }
    syscall::write(1, &input[i..]);
}
