use num_complex::Complex;

use crate::common::bi_signal_generation::bi_signal_modulation;
use crate::common::generate_wave::generate_wave;
use crate::modulators::ask::structs::modulation::Modulation;

impl Modulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_signal: f32) -> Modulation {
        let ask_off_signal = generate_wave(message_signal, sample_rate, samples_per_symbol as i32, 0, 0.0, 0.0, 0.0, 0.0);
        let ask_on_signal = generate_wave(message_signal, sample_rate, samples_per_symbol as i32, 0, 1.0, 0.0, 0.0, 0.0);

        Modulation { samples_per_symbol, sample_rate, ask_on_signal, ask_off_signal }
    }

    /// Modulate a radio signal using ASK
    ///
    /// # Arguments
    /// * `bin` - String of binary bits (ONLY 1s & 0s) to modulate (AKA Symbols)
    pub fn run(&self, bin: &[u8]) -> Vec<Complex<f32>>
    {
        bi_signal_modulation(bin, self.ask_off_signal.as_slice(), self.ask_on_signal.as_slice(), self.samples_per_symbol)
    }
}