use basm::platform::io::*;
pub fn main() {
    let mut reader = Reader::<128>::new();
    let mut writer = Writer::<128>::new();
    let mut x = [0; 5];
    let mut sum = 0;
    for i in 0..5 {
        x[i] = reader.u32();
        sum += x[i];
    }
    for i in 0..5 {
        for j in i+1..5 {
            if x[i] > x[j] {
                x.swap(i, j);
            }
        }
    }
    writer.println(sum / 5);
    writer.println(x[2]);
}