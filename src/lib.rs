// #![no_std]
// extern crate alloc;

use std::cmp::PartialEq;
use std::thread::spawn;
use crate::pipeline::{PipelineBuilder, PipelineSettings, PipelineType};

pub mod prelude{
    pub use crate::pipeline::Pipeline;
    pub use crate::pipeline::stages::*;
}

pub mod math;
pub mod pipeline;

pub trait AnyPipelineBuilder {
    fn get_run_mode(&self) -> Option<PipelineType>;
}

impl<I, O> AnyPipelineBuilder for PipelineBuilder<I, O> {
    fn get_run_mode(&self) -> Option<PipelineType>{
        self.running_pipeline_settings.pipeline_type
    }
}

#[derive(Default)]
pub struct App{
    pipeline: Vec<Box<dyn AnyPipelineBuilder>>,
    
    
}

impl App {
    pub fn new_pipeline<I: std::default::Default + 'static, O: std::default::Default + 'static, const ILen: usize, const OLen: usize>(&mut self) -> &Box<dyn AnyPipelineBuilder> {
        let builder: pipeline::PipelineBuilder<O, O> = PipelineBuilder::default();
        self.pipeline.push(Box::new(builder));
        
        self.pipeline.last().unwrap()
    }
}