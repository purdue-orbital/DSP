// #![no_std]
// extern crate alloc;

use std::cmp::PartialEq;
use std::sync::Arc;
use std::sync::Mutex;
use crate::pipeline::{PipelineBuilder, PipelineSettings, PipelineType};
use crate::prelude::FirstStage;

pub mod prelude{
    pub use crate::pipeline::Pipeline;
    pub use crate::pipeline::stages::*;
}

pub mod math;
pub mod pipeline;

// pub trait AnyPipelineBuilder {
//     fn get_run_mode(&self) -> Option<PipelineType>;
//     fn first_stage<I, O>(&self, input: Box<dyn FirstStage<I, O>>){
//         
//     };
// }
// 
// impl<I, O> AnyPipelineBuilder for PipelineBuilder<I, O> {
//     fn get_run_mode(&self) -> Option<PipelineType>{
//         self.running_pipeline_settings.pipeline_type
//     }
// }
// 
// #[derive(Default)]
// pub struct App{
//     pipeline: Vec<Arc<Mutex<Box<dyn AnyPipelineBuilder>>>>,
// }
// 
// impl App {
//     pub fn new_pipeline<I: std::default::Default + 'static, O: std::default::Default + 'static, const ILen: usize, const OLen: usize>(&mut self) -> Arc<Mutex<Box<dyn AnyPipelineBuilder>>> {
//         let builder: pipeline::PipelineBuilder<O, O> = PipelineBuilder::default();
//         self.pipeline.push(Arc::new(Mutex::new(Box::new(builder))));
//         
//         self.pipeline.last().unwrap().clone()
//     }
// }