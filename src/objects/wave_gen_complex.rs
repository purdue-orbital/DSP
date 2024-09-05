use core::f64::consts::PI;

use num::Complex;

use crate::objects::object::{Bus, DSPObject, Type};

#[derive(Clone, Copy)]
pub struct WaveStepGenComplex {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub sample_rate: f64,

    pub bus: Bus<'static>,

    pub time: f64,
}

impl WaveStepGenComplex {
    pub fn new(frequency: f64, amplitude: f64, phase: f64, sample_rate: f64) -> WaveStepGenComplex {
        WaveStepGenComplex {
            frequency,
            amplitude,
            phase,
            sample_rate,

            bus: Bus::new_complex(),

            time: 0.0,
        }
    }
}

impl DSPObject for WaveStepGenComplex {

    fn return_type(&self) -> Type {
        Type::Complex
    }

    fn input_type(&self) -> Type {
        Type::NONE
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        panic!("WaveStepGenComplex does not listen on a bus");
    }
    fn process(&mut self) {
        let phi = 2.0 * PI * self.frequency * self.time + self.phase;
        let value = Complex::new(self.amplitude * phi.sin(), self.amplitude * phi.cos());
        self.bus.trigger_complex(value);
        
        self.time += 1.0 / self.sample_rate;
    }
    fn start(&mut self) {
        loop {
            self.process();
        }
    }
}

