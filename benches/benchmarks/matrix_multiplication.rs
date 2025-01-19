use criterion::{black_box, criterion_group, Criterion};
use rustdsp::math::matrix::{matrix_multiply, matrix_multiply_3x3};

pub fn matrix_multiply_bench(c: &mut Criterion) {
    c.bench_function("matrix_multiply", |b| {
        b.iter(|| matrix_multiply(black_box(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]), black_box(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]])));
    });
}

pub fn matrix_multiply_bench_3x3(c: &mut Criterion) {
    c.bench_function("matrix_multiply", |b| {
        b.iter(|| matrix_multiply_3x3(black_box(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]), black_box(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]])));
    });
}

criterion_group!(benches, matrix_multiply_bench, matrix_multiply_bench_3x3);