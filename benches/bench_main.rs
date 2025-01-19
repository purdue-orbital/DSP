use criterion::criterion_main;
use crate::benchmarks::matrix_multiplication::benches;

mod benchmarks;

criterion_main! {
    benches,
}