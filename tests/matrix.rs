use rustdsp::math::matrix::{matrix_multiply, matrix_multiply_3x3};

#[test]
pub fn test_matrix_multiply() {
    let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let b = [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]];
    let expected = [[30.0, 24.0, 18.0], [84.0, 69.0, 54.0], [138.0, 114.0, 90.0]];
    assert_eq!(matrix_multiply(&a, &b), expected);
}

#[test]
pub fn test_matrix_multiply_3x3() {
    let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let b = [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]];
    let expected = [[30.0, 24.0, 18.0], [84.0, 69.0, 54.0], [138.0, 114.0, 90.0]];
    assert_eq!(matrix_multiply_3x3(&a, &b), expected);
}