use num::Complex;
use libm::{sqrtf, log2f, exp2f};

/// # Automatic Gain Controller
/// automatic gain control using basic algorithm found here:
/// https://wirelesspi.com/how-automatic-gain-control-agc-works/
/// 
/// # Examples
///
/// TODO
pub struct AGC<const BLOCK_SIZE: usize> {
    gain: f32,
    target: f32,
    enable_switch: bool,
    step_size: f32,
}

impl<const BLOCK_SIZE: usize> AGC<BLOCK_SIZE> {

    pub fn new(target: f32, step_size: f32) -> AGC<BLOCK_SIZE> {
        Self {
            gain: 1f32, 
            target: target, 
            enable_switch: false, 
            step_size: step_size}
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn set_step_size(&mut self, step_size: f32) {
        self.step_size = step_size;
    }

    pub fn set_initial_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn enable(&mut self) {
        self.enable_switch = true;
    }

    pub fn disable(&mut self) {
        self.enable_switch = false;
    }

    pub fn run(&mut self, samples: &[Complex<f32>; BLOCK_SIZE]) -> [Complex<f32>; BLOCK_SIZE] {
        // algorithm from https://wirelesspi.com/how-automatic-gain-control-agc-works/
        let mut output_signal = [Complex::new(0.0, 0.0); BLOCK_SIZE];

        if !self.enable_switch {
            output_signal.copy_from_slice(samples);
        } else {
            for n in 0..BLOCK_SIZE {
                output_signal[n] = samples[n].scale(self.gain);
                
                let norm = sqrtf(output_signal[n].re * output_signal[n].re + output_signal[n].im * output_signal[n].im); // can use approximation |z[n]| = sqrt(z_i^2 + z_q^2) approx. = |z_i| + |z_q|

                let error = self.target - norm; 
                self.gain += error * self.step_size;
            }
        }
        output_signal   
    }

    pub fn run_logorithmic(&mut self, samples: &[Complex<f32>; BLOCK_SIZE]) -> [Complex<f32>; BLOCK_SIZE] {
        // convergence time can be greatly reduced for some signals by using logorithms (at cost of computational efficiency)

        let mut output_signal = [Complex::new(0.0, 0.0); BLOCK_SIZE];

        if !self.enable_switch {
            output_signal.copy_from_slice(samples);
        } else {
            for n in 0..BLOCK_SIZE {
                output_signal[n] = samples[n].scale(exp2f(self.gain));
                
                let norm = sqrtf(output_signal[n].re * output_signal[n].re + output_signal[n].im * output_signal[n].im);

                let error = log2f(self.target / norm); 
                self.gain += error * self.step_size;
            }
        }
        output_signal   
    }

}

#[cfg(test)]
mod agc_tests {
    use super::*;
    use libm::{cosf, sinf};
    use core::f32::consts::PI;
    
    const ACCEPTED_DIFFERENCE: f32 = 0.01;
    const BLOCKS_BEFORE_CONVERGE: usize = 2;

    fn gen_samples<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize>(
        frequency: f32, 
        amplitude: f32, 
        sample_rate: f32,
    ) -> [[Complex<f32>; BLOCK_SIZE]; NUM_BLOCKS] {
        
        let mut samples = [[Complex::new(0.0, 0.0); BLOCK_SIZE]; NUM_BLOCKS];
        
        for i in 0..NUM_BLOCKS {
            for j in 0..BLOCK_SIZE {

                let t = ((i as f32) * (BLOCK_SIZE as f32) + (j as f32)) / sample_rate;
                let in_phase = amplitude * cosf(2.0 * PI * frequency * t);
                let quad = amplitude * sinf(2.0 * PI * frequency * t);

                samples[i][j] = Complex::new(in_phase, quad);
            }
        }

        samples
    }

    fn agc_amplify_samples<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize>(
        samples: &[[Complex<f32>; BLOCK_SIZE]; NUM_BLOCKS], 
        target: f32, 
        step_size: f32,
    ) -> [[Complex<f32>; BLOCK_SIZE]; NUM_BLOCKS] {

        let mut agc_obj: AGC<BLOCK_SIZE> = AGC::new(target, step_size);
        let mut amplified_samples = [[Complex::new(0.0, 0.0); BLOCK_SIZE]; NUM_BLOCKS];

        agc_obj.enable();
        agc_obj.set_initial_gain(0.0);

        for i in 0..NUM_BLOCKS {
            amplified_samples[i] = agc_obj.run_logorithmic(&samples[i]);
        }

        amplified_samples
    }

    #[test]
    fn agc_test_low_to_high() {

        // generate signals
        let f = 100.0;
        let a = 0.1;
        let f_s = 1000.0;
        let samples: [[Complex<f32>; 32]; 1024] = gen_samples(f, a, f_s);

        // amplify signals
        let target = 1.0;
        let step_size = 0.1;
        let amplified_samples = agc_amplify_samples(&samples, target, step_size);

        // verify amplified samples
        for i in BLOCKS_BEFORE_CONVERGE..1024 {
            for j in 0..32 {
                let t = ((i as f32) * 32.0 + (j as f32)) / f_s;
                let perfect_val = target * cosf(2.0 * PI * f * t);
                let diff = (amplified_samples[i][j].re - perfect_val).abs();
                assert!(diff < ACCEPTED_DIFFERENCE);
            }
        }
    }

    #[test]
    fn agc_test_high_to_low() {

        // generate signals
        let f = 100.0;
        let a = 10.0;
        let f_s = 1000.0;
        let samples: [[Complex<f32>; 32]; 1024]  = gen_samples(f, a, f_s);

        // amplify signals
        let target = 1.0;
        let step_size = 0.1;
        let amplified_samples = agc_amplify_samples(&samples, target, step_size);

        // verify amplified samples
        for i in BLOCKS_BEFORE_CONVERGE..1024 {
            for j in 0..32 {
                let t = ((i as f32) * 32.0 + (j as f32)) / f_s;
                let perfect_val = target * cosf(2.0 * PI * f * t);
                let diff = (amplified_samples[i][j].re - perfect_val).abs();
                assert!(diff < ACCEPTED_DIFFERENCE);
            }
        }
    }

}