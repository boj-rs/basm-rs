use basm::platform::io::{Print, Reader, ReaderTrait, Writer};
use basm::geometry::{point,ccw};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.i32();
    let b = reader.i32();
    let c = reader.i32();
    let d = reader.i32();
    let e = reader.i32();
    let f = reader.i32();
    let p1 = point::Point::new(a,b);
    let p2 = point::Point::new(c,d);
    let p3 = point::Point::new(e,f);
    let res = ccw(p1,p2,p3);
    if res < 0 {
        writer.str("-1");
    } else if res>0 {
        writer.str("1");
    } else {
        writer.str("0");
    }
}
