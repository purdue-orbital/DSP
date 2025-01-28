use async_trait::async_trait;
use crate::pipeline::PipelineSettings;
use crate::prelude::{Data, DataKind, LastStage, Stage};

#[derive(Default)]
pub struct Print {
    empty_data: Data,
}

impl<I,O>  Stage<I,O> for Print {
    fn configure(&mut self, data: &mut PipelineSettings) {
        data.prev_stage_output = DataKind::F64;
    }

    fn process(&mut self, data: &Data, output: &mut Data) {
        println!("{:?}", data.get_f64_data());
    }

    fn get_output_data_type(&self) -> DataKind {
        DataKind::Empty
    }

    fn get_input_data_type(&self) -> DataKind {
        DataKind::F64
    }
}

impl<I,O: Send + Sync> LastStage<I,O> for Print {
}