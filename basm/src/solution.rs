use basm::platform::io::{Print, Reader, ReaderTrait, Writer};
use basm::geometry::{point,ccw};
pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = reader.i128();
    let b = reader.i128();
    let c = reader.i128();
    let d = reader.i128();
    let e = reader.i128();
    let f = reader.i128();
    let p1 = point::Point::new(a,b);
    let p2 = point::Point::new(c,d);
    let p3 = point::Point::new(e,f);
    let res = ccw(p1,p2,p3);
    if res < 0.0 {
        writer.str("-1");
    } else if res>0.0 {
        writer.str("1");
    } else {
        writer.str("0");
    }
}
