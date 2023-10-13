use std::f32::consts::PI;

use num_complex::Complex;

use crate::common::constellation::{Constellation, ConstellationPoint};
use crate::modulators::bpsk::structs::modulation::Modulation;

impl Modulation {
    pub fn new(samples_per_symbol: usize, sample_rate: f32, message_signal: f32) -> Modulation {
        let mut constellation = Constellation::new(message_signal, sample_rate, samples_per_symbol);

        let bin_zero = ConstellationPoint::new(0, 0.0, 0.0, 1.0, 0.0);
        let bin_one = ConstellationPoint::new(1, PI, 0.0, 1.0, 0.0);

        constellation.add_state(&bin_zero);
        constellation.add_state(&bin_one);

        Modulation { samples_per_symbol, sample_rate, constellation }
    }

    /// Modulate a radio signal using bpsk
    ///
    /// # Arguments
    /// * `bin` - String of binary bits (ONLY 1s & 0s) to modulate (AKA Symbols)
    pub fn run(&self, bin: &[u8]) -> Vec<Complex<f32>> {

        // explode bin into bits
        let mut corrected = vec![];
        for &x in bin {
            for y in (0..8).rev() {
                corrected.push(((x >> y) & 1) as u128);
            }
        }

        // run
        self.constellation.generate(corrected.as_slice())
    }
}