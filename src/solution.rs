use basm::io;

#[inline(always)]
pub fn main() {
    const LIST: [&[u8]; 2] = [b"ki", b"wi"];
    let buf = [0u8; 1];
    let n = buf.as_ptr() as usize;
    let mut writer = io::Writer::<{ 1 << 15 }>::new();
    let mut i = n % 2;
    let mut j = 1 - i;
    if i > j {
        (i, j) = (j, i);
    }
    writer.write(LIST[i]);
    writer.write(LIST[j]);
    writer.write(b"\n");
}