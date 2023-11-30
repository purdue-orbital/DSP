use num_complex::Complex;


// /// builds root raised filter. This was implemented using this paper https://engineering.purdue.edu/~ee538/SquareRootRaisedCosine.pdf
// pub fn build_rrc(samples_per_a_symbol:usize, roll_off:f32, baud_rate:f32) -> Vec<f32>{
//     let time_increment = 1.0 / baud_rate;
//     let mut t = -1.0 *  time_increment * (samples_per_a_symbol / 2) as f32;
//     let mut arr = Vec::with_capacity(samples_per_a_symbol);
//
//     for _ in 0..samples_per_a_symbol {
//         arr.push(
//             // lord have mercy
//             ((2.0 * roll_off) / (PI * time_increment.sqrt())) * ((((1.0 + roll_off) * PI * (t / time_increment)).cos() + (((1.0 - roll_off) * PI * (t / time_increment)).sin())) / (1.0 - (4.0 * roll_off * (t / time_increment)).powi(2)))
//         );
//
//         t += time_increment
//     }
//
//     arr
// }


pub struct SymbolSync{
    samples_per_a_symbol: usize
}

impl SymbolSync{
    /// Run symbol sync. Will fail if samples.len != samples per a symbol
    pub fn run(&mut self, samples: &[Complex<f32>]) -> Complex<f32> {
        assert_eq!(samples.len(),self.samples_per_a_symbol);

        samples[0]
    }

    /// New symbol sync loop instance
    pub fn new(samples_per_a_symbol: usize) -> SymbolSync{
        SymbolSync{
            samples_per_a_symbol
        }
    }
}