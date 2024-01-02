#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::similar_names)]

pub mod nttcore;
pub mod multiply;
pub use multiply::multiply_u64;
pub mod polymul;
pub use polymul::polymul_u64;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_multiply_u64() {
        assert_eq!(vec![6, 0], multiply_u64(&[2], &[3]));
        assert_eq!(
            vec![338457285041528512, 873741774122123434, 1614279433659886989, 2232618956921417485],
            multiply_u64(
                &[2745895686766400144, 7520886206723962441],
                &[9006621652498007052, 5476023620144815056]
            )
        );
    }
    #[test]
    fn check_polymul_u64() {
        assert_eq!(vec![6], polymul_u64(&[2], &[3], 0));
        assert_eq!(
            vec![388856412, 499682766, 44258992],
            polymul_u64(
                &[2745895686766400144, 7520886206723962441],
                &[9006621652498007052, 5476023620144815056],
                1000000007
            )
        );
        assert_eq!(
            vec![16889012037339467114, 15272081445206089825, 5723988045866129237],
            polymul_u64(
                &[2745895686766400144, 7520886206723962441],
                &[9006621652498007052, 5476023620144815056],
                18446744073606613507
            )
        );
    }
}