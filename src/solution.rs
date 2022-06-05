use crate::io;

#[inline(always)]
pub fn main() {
    // 여기에 코드 입력...
    // 아래는 예시 코드입니다
    let mut reader = io::Reader::<{ 1 << 15 }>::new();
    let mut writer = io::Writer::<{ 1 << 15 }>::new();
    let a = reader.next_uint();
    let b = reader.next_uint();
    writer.write_usize(a + b);
    // 여기까지 예시 코드입니다
}
