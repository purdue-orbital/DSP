use cordic::CordicNumber;
use crate::stages::Stage;

pub struct WaveGen<T: CordicNumber, const len: usize> {
    phi: T,
    delta: T,
    
    amplitude: T,
    phase: T,
    time: T,
}

impl<T: CordicNumber, const len: usize> WaveGen<T, { len }> {
    pub fn new(freq: T, ampl: T, phase: T, sample_rate: T) -> Self {
        let delta = T::one() / sample_rate;
        let phi = (T::one() + T::one()) * T::pi() * freq;
        
        WaveGen {
            phi,
            delta,
            
            amplitude: ampl,
            phase,
            
            time: T::zero(),
        }
    }
}

impl<T: CordicNumber, const len: usize> Stage for WaveGen<T, { len }> {
    type Input = ();
    type Output = [T; len];

    async fn process(&mut self, _input: Self::Input) -> Self::Output {
        let mut result = [T::zero(); len];
        
        for i in 0..len {
            let e = cordic::sin(self.phi * self.time + self.phase) * self.amplitude;
            self.time += self.delta;
            
            result[i] = e;
        }
        
        result
    }
}

