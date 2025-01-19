use core::ops::{Add, AddAssign, Mul};
use crate::math::matrix::{matrix_multiply, transpose};

// pub fn cross_correlation<const n: usize, const p:usize, T: Mul<Output = T>+Add+Copy+AddAssign+From<u8>>(a: &[[T; n]; 1], b: &[[T; p]; 1]) -> T {
//     let b_t = transpose(b);
//     
//     let test = matrix_multiply(a, &b_t)[0][0];
//     
//     test
// }