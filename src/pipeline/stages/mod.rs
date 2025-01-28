use num::Num;
use crate::pipeline::PipelineSettings;

pub mod wave_gen;
pub mod wave_gen_complex;

pub struct Data{
    
}

pub trait Stage<I,O> {
    fn configure(&mut self, data: &mut PipelineSettings<I, O>);
    fn process(&mut self);
}

pub trait FirstStage<I,O>: Stage<I,O> {
    fn run(&mut self, data: Vec<I>);
}

pub trait LastStage<I,O>: Stage<I,O> {
    fn run(&mut self) -> Vec<O>;
}