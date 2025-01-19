use core::ops::{Add, AddAssign, Mul};

pub fn matrix_multiply<const m: usize, const n: usize, const p: usize, T: Mul<Output = T>+Add+Copy+AddAssign+From<u8>>(a: &[[T; n]; m], b: &[[T; p]; n]) -> [[T; p]; m] {
    let mut result = [[T::from(0u8); p]; m];

    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

pub fn matrix_multiply_3x3<T: Mul<Output = T>+Add<Output = T>+Copy+AddAssign+From<u8>>(a: &[[T; 3]; 3], b: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    [
        [a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0], a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1], a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2]],
        [a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0], a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1], a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2]],
        [a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0], a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1], a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2]],
    ]
}

pub fn transpose<const m: usize, const n: usize, T: Copy>(a: &[[T; n]; m]) -> [[T; m]; n] {
    let mut result = [[a[0][0]; m]; n];

    for i in 0..m {
        for j in 0..n {
            result[j][i] = a[i][j];
        }
    }

    result
}