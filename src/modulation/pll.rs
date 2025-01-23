use libm::{cosf, sinf};
use core::f32::consts::PI;
use num::Complex;

type C32 = Complex<f32>;

pub enum PLLState {
    Free,
    Capture,
    Lock,
}

/// Phase locked loop that acts on a block of samples, or single samples
pub struct PLL<const BLOCK_SIZE: usize = 8> {
    sample_rate: f32,
    angular_freq: f32,
    amplitude: f32,
    error_detector_gain_inverse: f32,
    gain_prop: f32,
    gain_int: f32,
    integrated_err: f32,
    phase_err_estimate: f32,
    current_phase: f32,
    state: PLLState,
}

impl <const BLOCK_SIZE: usize> PLL<BLOCK_SIZE> {
    
    /// Make new pll by specifying frequency, amplitude, sample rate, and the proportional + integral low pass filter constants
    pub fn new_with_gain_consts(freq: f32, amplitude: f32, sample_rate: f32, gain_prop: f32, gain_int: f32) -> PLL<BLOCK_SIZE> {
        PLL {
            sample_rate: sample_rate,
            angular_freq: 2.0 * PI * freq / sample_rate,
            amplitude: amplitude,
            error_detector_gain_inverse: 1.0 / amplitude / amplitude,
            gain_prop: gain_prop,
            gain_int: gain_int,
            integrated_err: 0.0,
            phase_err_estimate: 0.0,
            current_phase: 0.0,
            state: PLLState::Free,
        }
    }

    /// Make new pll by specifying frequency, amplitude, sample rate, and the proportional + integral low pass filter determinents
    /// Usual values for parameters
    /// * damping factor: between 0.5 and 2.0
    /// * noise bandwidth: between 1% and 5% of the sample rate
    pub fn new(freq: f32, amplitude: f32, sample_rate: f32, damping_factor: f32, noise_bandwidth: f32) -> PLL<BLOCK_SIZE> {
        let mut result = PLL {
            sample_rate: sample_rate,
            angular_freq: 2.0 * PI * freq / sample_rate,
            amplitude: amplitude,
            error_detector_gain_inverse: 1.0 / amplitude / amplitude,
            gain_prop: 0.0,
            gain_int: 0.0,
            integrated_err: 0.0,
            phase_err_estimate: 0.0,
            current_phase: 0.0,
            state: PLLState::Free,
        };

        result.tune(damping_factor, noise_bandwidth);
        result
    }

    /// tune the gain constants with desired damping factor and effective noise bandwidth
    /// 
    /// ```
    /// use superdsp::modulation::pll::PLL;
    /// 
    /// let mut pll: PLL<32> = PLL::new_with_gain_consts(100.0, 1.0, 500.0, 0.0, 0.0);
    /// pll.tune(1.0, 25.0);
    /// 
    /// let (gain_prop, gain_int) = pll.gain_consts();
    /// 
    /// assert!((gain_prop - 0.16).abs() < f32::EPSILON);
    /// assert!((gain_int - 0.0064).abs() < f32::EPSILON);
    /// ```
    /// 
    /// Usual values for parameters
    /// * damping factor: between 0.5 and 2.0
    /// * noise bandwidth: between 1% and 5% of the sample rate
    /// 
    /// for more details on these gain constants and their derivation, see https://wirelesspi.com/phase-locked-loop-pll-in-a-software-defined-radio-sdr/
    pub fn tune(&mut self, damping_factor: f32, noise_bandwidth: f32) {
        self.gain_prop = 4.0 * damping_factor * noise_bandwidth / 
            (self.sample_rate * (damping_factor + 0.25 / damping_factor));
        self.gain_int = 4.0 * noise_bandwidth * noise_bandwidth /
            ( self.sample_rate * (damping_factor + 0.25 / damping_factor) * 
            self.sample_rate * (damping_factor + 0.25 / damping_factor));
    } 

    pub fn set_gain_consts(&mut self, gain_prop: f32, gain_int: f32) {
        self.gain_prop = gain_prop;
        self.gain_int = gain_int;
    }

    pub fn gain_consts(&self) -> (f32, f32) {
        (self.gain_prop, self.gain_int)
    }

    pub fn set_state(&mut self, state: PLLState) {
        self.state = state;
    }

    pub fn set_free(&mut self) {
        self.state = PLLState::Free;
    }

    pub fn set_capture(&mut self) {
        self.state = PLLState::Capture;
    }

    pub fn set_lock(&mut self) {
        self.state = PLLState::Lock;
    }

    pub fn run_free(&mut self) -> [C32; BLOCK_SIZE] {
        let mut output_samples = [Complex::new(0.0, 0.0); BLOCK_SIZE];

        for n in 0..BLOCK_SIZE {
            output_samples[n] = Complex::new(cosf(self.current_phase), sinf(self.current_phase)).scale(self.amplitude);
            self.current_phase += self.angular_freq;
        }
        output_samples
    }

    pub fn run_locked(&mut self) -> [C32; BLOCK_SIZE] {
        let mut output_samples = [Complex::new(0.0, 0.0); BLOCK_SIZE];

        for n in 0..BLOCK_SIZE {
            let theta = self.current_phase + self.phase_err_estimate;
            output_samples[n] = Complex::new(cosf(theta), sinf(theta)).scale(self.amplitude);
            self.current_phase += self.angular_freq;
        }

        output_samples
    }

    pub fn run_capture(&mut self, input_samples: &[C32; BLOCK_SIZE]) -> [C32; BLOCK_SIZE] {
        let mut output_samples = [Complex::new(0.0, 0.0); BLOCK_SIZE];

        for n in 0..BLOCK_SIZE {
            
            // generate output samples
            let theta = self.current_phase + self.phase_err_estimate;
            output_samples[n] = Complex::new(cosf(theta), -sinf(theta)).scale(self.amplitude);
            
            // error detection
            let raw_error = (input_samples[n] * output_samples[n]).scale(self.error_detector_gain_inverse);

            // generate error signal
            let error_signal = {
                if raw_error.re > 0.0 {
                    raw_error.im
                } else {
                    // accelerate locking by increasing error signal if phase difference exceeds a quarter cycle
                    2f32.copysign(raw_error.im) - raw_error.im
                }
            };

            // low pass PI filter
            self.integrated_err += error_signal;
            self.integrated_err = self.integrated_err.clamp(-3.0, 3.0);

            self.phase_err_estimate += self.gain_prop * error_signal + self.gain_int * self.integrated_err;
            
            // update output_signal phase
            self.current_phase += self.angular_freq;
        }

        output_samples
    }

    pub fn run(&mut self, input_samples: &[C32; BLOCK_SIZE]) -> [C32; BLOCK_SIZE] {
        match self.state {
            PLLState::Capture => self.run_capture(input_samples),
            PLLState::Free => self.run_free(),
            PLLState::Lock => self.run_locked(),
        }
    }
}

#[cfg(test)]
mod pll_tests {

    use super::*;
    use libm::{cosf, sinf, atanf};
    use core::f32::consts::PI;
    use rand::random;

    const TEST_BLOCK_SIZE: usize = 32;
    const TEST_NUM_BLOCKS: usize = 256;
    const ACCEPTED_PHASE_ERROR: f32 = PI / 8.0;
    const BLOCKS_BEFORE_LOCK: usize = 1;

    fn gen_samples<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize>(
        frequency: f32, 
        amplitude: f32,
        initial_phase: f32, 
        sample_rate: f32,
    ) -> [[C32; BLOCK_SIZE]; NUM_BLOCKS] {
        
        let mut samples = [[Complex::new(0.0, 0.0); BLOCK_SIZE]; NUM_BLOCKS];
    
        for i in 0..NUM_BLOCKS {
            for j in 0..BLOCK_SIZE {
    
                let t = ((i as f32) * (BLOCK_SIZE as f32) + (j as f32)) / sample_rate;
                let in_phase = amplitude * cosf(2.0 * PI * frequency * t + initial_phase);
                let quad = amplitude * sinf(2.0 * PI * frequency * t + initial_phase);
    
                samples[i][j] = Complex::new(in_phase, quad);
            }
        }
    
        samples
    }
    
    fn gen_samples_freq_noise<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize>(
        frequency: f32, 
        amplitude: f32,
        initial_phase: f32, 
        sample_rate: f32,
        noise_bandwidth: f32
    ) -> [[C32; BLOCK_SIZE]; NUM_BLOCKS] {
        
        let mut samples = [[Complex::new(0.0, 0.0); BLOCK_SIZE]; NUM_BLOCKS];
        let mut phase = initial_phase;
    
        for i in 0..NUM_BLOCKS {
            for j in 0..BLOCK_SIZE {
    
                let in_phase = amplitude * cosf(phase);
                let quad = amplitude * sinf(phase);
    
                samples[i][j] = Complex::new(in_phase, quad);
    
                let noisy_freq: f32 = frequency * (1.0 + noise_bandwidth / 2.0 - noise_bandwidth * random::<f32>());
    
                phase += 2.0 * PI * noisy_freq / sample_rate;
            }
        }
    
        samples
    }
    
    fn pll_run_capture<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize>(
        mut pll: PLL<BLOCK_SIZE>, 
        samples: &[[C32; BLOCK_SIZE]; NUM_BLOCKS],
    ) -> [[C32; BLOCK_SIZE]; NUM_BLOCKS] {
        let mut pll_output = [[Complex::new(0.0, 0.0); BLOCK_SIZE]; NUM_BLOCKS];
        
        pll.set_capture();

        for i in 0..NUM_BLOCKS {
            pll_output[i] = pll.run(&samples[i]);
        }

        pll_output
    }

    fn lock_from(f: f32, a: f32, f_s: f32, phase: f32) {

        let locked_sample_ideal: [[C32; TEST_BLOCK_SIZE]; TEST_NUM_BLOCKS] = gen_samples(f, a, phase, f_s);

        let pll_obj: PLL<TEST_BLOCK_SIZE> = PLL::new(f, a, f_s, 0.707, f_s * 0.05);
        let samples = pll_run_capture(pll_obj, &locked_sample_ideal);

        for i in 0..TEST_NUM_BLOCKS {
            for j in 0..TEST_BLOCK_SIZE {
                let err_complex = samples[i][j] * locked_sample_ideal[i][j];

                let mut true_error = atanf(err_complex.im / err_complex.re);
                if err_complex.re.is_sign_negative() {
                    true_error += PI.copysign(err_complex.im);
                }

                if i >= BLOCKS_BEFORE_LOCK {
                    assert!(true_error.abs() < ACCEPTED_PHASE_ERROR, "sample {}: phase_err: {:7.3}", i * TEST_BLOCK_SIZE + j, true_error);
                }
            }
        }
    }

    fn lock_from_noisy(f: f32, a: f32, f_s: f32, phase: f32, pll_damping_factor: f32, pll_noise_bandwidth: f32, noisiness: f32) {

        let locked_sample_ideal: [[C32; TEST_BLOCK_SIZE]; TEST_NUM_BLOCKS] = gen_samples_freq_noise(f, a, phase, f_s, noisiness);

        let pll_obj: PLL<TEST_BLOCK_SIZE> = PLL::new(f, a, f_s, pll_damping_factor, pll_noise_bandwidth);
        let samples = pll_run_capture(pll_obj, &locked_sample_ideal);
        
        for i in 0..TEST_NUM_BLOCKS {
            for j in 0..TEST_BLOCK_SIZE {
                let err_complex = samples[i][j] * locked_sample_ideal[i][j];

                let mut true_error = atanf(err_complex.im / err_complex.re);
                if err_complex.re.is_sign_negative() {
                    true_error += PI.copysign(err_complex.im);
                }

                if i >= BLOCKS_BEFORE_LOCK {
                    assert!(true_error.abs() < ACCEPTED_PHASE_ERROR, "phase error out of acceptable range of {ACCEPTED_PHASE_ERROR:.3}");
                }
            }
        }
    }

    #[test]
    fn lock_from_180() {
        lock_from(1000.0, 1.0, 4000.0, PI);
    }

    #[test]
    fn lock_from_180_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, PI, 1.5, 0.05 * 4000.0, 0.05);
    }

    #[test]
    fn lock_from_135_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, 0.75 * PI, 1.5, 0.05 * 4000.0, 0.05);
    }

    #[test]
    fn lock_from_90_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, 0.5 * PI, 1.5, 0.05 * 4000.0, 0.05);
    }

    #[test]
    fn lock_from_45_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, 0.25 * PI, 1.5, 0.05 * 4000.0, 0.05);
    }

    #[test]
    fn lock_from_0_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, 0.0, 1.5, 0.05 * 4000.0, 0.05);
    }

    #[test]
    fn lock_from_neg45_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, -0.25 * PI, 1.5, 0.05 * 4000.0, 0.05);
    }
    #[test]
    fn lock_from_neg90_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, -0.5 * PI, 1.5, 0.05 * 4000.0, 0.05);
    }
    #[test]
    fn lock_from_neg135_noisy() {
        lock_from_noisy(1000.0, 1.0, 4000.0, -0.75 * PI, 1.5, 0.05 * 4000.0, 0.05);
    }

}