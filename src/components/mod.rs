pub mod nco;
pub mod loop_filter;
pub mod lpf;

/// this will take a sample and either set it to -1.0 or 1.0 .
pub fn threshold(sample:f32) -> f32 {
    if sample.is_sign_negative() {
        -1.0
    }else {
        1.0
    }
}