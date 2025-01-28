use std::f64::consts::PI;
use std::fmt::Debug;
use crate::pipeline::PipelineSettings;
use crate::prelude::{Data, DataKind, FirstStage, Stage};

pub struct WaveGen<I> {
    phi: f64,
    sample_rate: f64,
    
    num_taps: usize,
    
    time: f64,

    in_channel_tx: tokio::sync::broadcast::Sender<Vec<I>>,
    in_channel: tokio::sync::broadcast::Receiver<Vec<I>>,
}

impl<I: Clone> Default for WaveGen<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Clone> WaveGen<I> {
    pub fn new() -> Self {
        let (in_channel_tx, in_channel) = tokio::sync::broadcast::channel::<Vec<I>>(1);
        
        Self {
            phi: 0.0,
            sample_rate: 0.0,
            num_taps: 0,
            time: 0.0,
            in_channel_tx,
            in_channel,
        }
    }
}

impl<I: Clone + Debug + std::marker::Send, O> Stage<I, O> for WaveGen<I> {
    fn configure(&mut self, data: &mut PipelineSettings) {
        // make sure the required fields are present
        assert_eq!(data.prev_stage_output, DataKind::Empty);
        
        let freq = data.frequency.expect("wave_gen needs frequency");
        let sample_rate = data.sample_rate.expect("wave_gen needs sample rate");
        
        self.num_taps = data.num_taps.expect("wave_gen needs num_taps");
        
        self.phi = 2.0 * PI * freq;
        self.sample_rate = sample_rate;
    }

    fn process(&mut self, data: &Data, output: &mut Data) {
        let data = unsafe { output.f64_data.as_mut().unwrap_unchecked() };
        
        for x in data{
            *x = (self.phi * self.time).sin(); 
            
            self.time += 1.0 / self.sample_rate;
            self.time %= 1.0;
        }
    }

    fn get_output_data_type(&self) -> DataKind {
        DataKind::F64
    }

    fn get_input_data_type(&self) -> DataKind {
        DataKind::Empty
    }
}


impl<I: Clone + Debug + std::marker::Send,O> FirstStage<I,O> for WaveGen<I> {
}