// Test Triangle Window

use superdsp::filters_and_windows::hamming;

const N_SHORT: usize = 16;
const N_LONG: usize = 64;
#[test]
pub fn triangle_window_short(){
    let expected = [
        0.08696,
        0.12171,
        0.22067,
        0.36877,
        0.54348,
        0.71818,
        0.86629,
        0.96525,
        1.0,
        0.96525,
        0.86629,
        0.71818,
        0.54348,
        0.36877,
        0.22067,
        0.12171
    ];
    
    let window = hamming::triangle_window::<N_SHORT>(N_SHORT, 0);
    assert_eq!(window.len(), N_SHORT);
    
    // make sure the window is correct to 3 decimal places (floating point error)
    for i in 0..N_SHORT {
        let expected_value = expected[i];
        let got_value = window[i];
        println!("Testing {i}, expecting {expected_value} and got {got_value}."); 
        assert!((window[i] - expected[i]).abs() < 0.001);
    }
}

#[test]
fn triangle_window_long(){
    let expected = [
        0.0,
        0.03125,
        0.0625,
        0.09375,
        0.125,
        0.15625,
        0.1875,
        0.21875,
        0.25,
        0.28125,
        0.3125,
        0.34375,
        0.375,
        0.40625,
        0.4375,
        0.46875,
        0.5,
        0.53125,
        0.5625,
        0.59375,
        0.625,
        0.65625,
        0.6875,
        0.71875,
        0.75,
        0.78125,
        0.8125,
        0.84375,
        0.875,
        0.90625,
        0.9375,
        0.96875,
        1.0,
        0.96875,
        0.9375,
        0.90625,
        0.875,
        0.84375,
        0.8125,
        0.78125,
        0.75,
        0.71875,
        0.6875,
        0.65625,
        0.625,
        0.59375,
        0.5625,
        0.53125,
        0.5,
        0.46875,
        0.4375,
        0.40625,
        0.375,
        0.34375,
        0.3125,
        0.28125,
        0.25,
        0.21875,
        0.1875,
        0.15625,
        0.125,
        0.09375,
        0.0625,
        0.03125
    ];
    
    let window = hamming::triangle_window::<N_LONG>(N_LONG, 0);
    assert_eq!(window.len(), N_LONG);
    
    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_LONG {
        println!("{}: {}", i, window[i]);
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}

