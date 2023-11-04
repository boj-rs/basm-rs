use basm::platform::io::{Reader, Writer};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    const LIST: [&[u8]; 2] = [b"ki", b"wi"];
    let buf = [0u8; 1];
    let n = reader.usize();
    for _ in 0..n {
        let v = buf.as_ptr() as usize;
        let i = (v + reader.usize()) % 2;
        let j = 1 - i;
        writer.bytes(LIST[i]);
        writer.bytes(LIST[j]);
        writer.byte(b'\n');
    }
}