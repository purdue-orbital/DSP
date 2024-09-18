// This function is based on the triangle filter function from wikipedia found here: https://en.wikipedia.org/wiki/Window_function

/// Creates a hamming window of size N
/// Formula: f(x) = (25.0/46.0) - ((1 - (25.0/46.0)) * cos((2*pi*(n - offset))/N)) for n in 0..N
/// 
/// Where:
/// L can be N, N+1, or N+2
/// o is the bin offset
/// 
/// # Arguments
/// N: usize - Size of the filter
/// l: usize - Filter setting (N, N+1, or N+2)
/// offset: usize - Bin offset
/// 
/// # Returns
/// [f32; N] - Triangle window\
pub fn hamming_window<const N: usize>(l: usize, offset: usize) -> [f32; N] {
    let mut filter = [0.0; N];

    let a_nought: f32 = 25.0/46.0;
    let float_large_n: f32 = N as f32;
    
    for n in 0..N {
        let float_small_n = n as f32 - offset as f32;
        filter[n] = a_nought - (1.0-a_nought) * libm::cosf((2.0*core::f32::consts::PI*float_small_n)/float_large_n);
    }
    
    filter
}