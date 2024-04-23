use rustdsp::math::{expj, FRAC_PI_2, PI, sincos};
// 
// #[test]
// pub fn sincos_test() {
//     let mut e_sum = 0.0;
// 
//     for x in -100..100 {
//         let i = PI * (x as f32 / 100.0);
// 
//         let sincos = sincos(i);
// 
//         let sin = sincos.0;
//         let cos = sincos.1;
// 
//         e_sum += (sin - i.sin()).abs();
//         e_sum += (cos - i.cos()).abs();
//     }
// 
//     assert!(e_sum < 3.0);
// }

#[test]
pub fn exp_test() {
    let c = expj(PI);
    assert_eq!(c.re, -i16::MAX);
    assert_eq!(c.im, 0);

    let c = expj(2 * PI);
    assert_eq!(c.re, i16::MAX);
    assert_eq!(c.im, 0);

    let c = expj(FRAC_PI_2);
    assert_eq!(c.re, 0);
    assert_eq!(c.im, i16::MAX);

    let c = expj(3 * FRAC_PI_2);
    assert_eq!(c.re, 0);
    assert_eq!(c.im, -i16::MAX);

    let c = expj(-FRAC_PI_2);
    assert_eq!(c.re, 0);
    assert_eq!(c.im, -i16::MAX);

    let c = expj(-PI);
    assert_eq!(c.re, -i16::MAX);
    assert_eq!(c.im, 0);
}