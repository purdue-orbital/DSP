use std::f64::consts::PI;
use crate::pipeline::PipelineSettings;
use crate::prelude::{FirstStage, Stage};

pub struct WaveGen {
    phi: f64,
    sample_rate: f64,
    
    num_taps: usize,
    
    time: f64,
    
    in_channel: tokio::sync::broadcast::Receiver<Vec<()>>,
    out_channel: tokio::sync::broadcast::Sender<Vec<f64>>,
}

impl WaveGen {
    

}

impl<I, O> Stage<I, O> for WaveGen {
    fn configure(&mut self, data: &mut PipelineSettings<I, O>) {
        let freq = data.frequency.expect("wave_gen needs frequency");
        let sample_rate = data.sample_rate.expect("wave_gen needs sample rate");
        
        self.num_taps = data.num_taps.expect("wave_gen needs num_taps");
        
        self.phi = 2.0 * PI * freq / sample_rate;
        self.sample_rate = sample_rate;
    }

    fn process(&mut self) {
        for _ in 0..self.num_taps {
            
            
            self.time += 1.0 / self.sample_rate;
        }
    }
}

impl<I,O> FirstStage<I,O> for WaveGen {
    fn run(&mut self, data: Vec<I>) {
        todo!()
    }
}