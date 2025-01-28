pub mod stages;

use std::fmt::Debug;
use crate::prelude::*;

use tokio::sync::broadcast::{Sender, Receiver};


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PipelineType{
    Loop,
    OnSend,
}

#[derive(Default)]
pub struct PipelineSettings<I,O> {
    pub pipeline_type: Option<PipelineType>,
    
    frequency: Option<f64>,
    sample_rate: Option<f64>,

    sps: Option<usize>,
    
    num_taps: Option<usize>,

    input_channel: Option<Sender<Vec<I>>>,
    output_channel: Option<Receiver<Vec<O>>>,
}

#[derive(Default)]
pub struct PipelineBuilder<I,O> {
    pub running_pipeline_settings: PipelineSettings<I, O>,
    
    first_stage: Option<Box<dyn FirstStage<I,O>>>,
    
    last_stage: Option<Box<dyn LastStage<I,O>>>,
    
    stages: Vec<Box<dyn Stage<I,O>>>,
}

impl<I,O> PipelineBuilder<I,O> {
    pub fn add_stage(&mut self, stage: Box<dyn Stage<I,O>>) -> &mut Self {
        self.stages.push(stage);
        
        self
    }
    
    pub fn add_first_stage(&mut self, stage: Box<dyn FirstStage<I,O>>) -> &mut Self {
        self.first_stage = Some(stage);
        self
    }
    
    pub fn add_last_stage(&mut self, stage: Box<dyn LastStage<I,O>>) -> &mut Self {
        self.last_stage = Some(stage);
        self
    }
    
    pub fn build(&mut self) -> Pipeline<I,O> {
        Pipeline{
            first_stage_in: self.running_pipeline_settings.input_channel.take().unwrap(),
            last_stage_out: self.running_pipeline_settings.output_channel.take().unwrap(),
        }
    }
}

pub struct Pipeline<I,O> {
    first_stage_in: Sender<Vec<I>>,
    last_stage_out: Receiver<Vec<O>>,
}
impl<I: Clone + Send + Sync + Debug + 'static,O: Clone> Pipeline<I, O> {
    pub fn send(&self, data: Vec<I>) -> anyhow::Result<()> {
        self.first_stage_in.send(data)?;

        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<Vec<O>> {
        Ok(self.last_stage_out.recv().await?)
    }
}