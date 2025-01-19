use cordic::CordicNumber;
use num_complex::Complex;
use crate::stages::Stage;

pub struct WaveGenComplex<T: CordicNumber, const len: usize> {
    phi: T,
    delta: T,

    amplitude: T,
    phase: T,
    time: T,
}

impl<T: CordicNumber, const len: usize> WaveGenComplex<T, { len }> {
    pub fn new(freq: T, ampl: T, phase: T, sample_rate: T) -> Self {
        let delta = T::one() / sample_rate;
        let phi = (T::one() + T::one()) * T::pi() * freq;

        WaveGenComplex {
            phi,
            delta,

            amplitude: ampl,
            phase,

            time: T::zero(),
        }
    }
}

impl<T: CordicNumber, const len: usize> Stage for WaveGenComplex<T, { len }> {
    type Input = ();
    type Output = [Complex<T>; len];

    async fn process(&mut self, _input: Self::Input) -> Self::Output {
        let mut result = [Complex::new(T::zero(),T::zero()); len];

        for i in 0..len {
            let e = cordic::sin_cos((self.phi * self.time + self.phase) * self.amplitude);
            self.time += self.delta;

            result[i].im = e.0;
            result[i].re = e.1;
        }

        result
    }
}