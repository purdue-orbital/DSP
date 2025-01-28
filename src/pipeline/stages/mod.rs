use async_trait::async_trait;
use num::Num;
use crate::pipeline::PipelineSettings;

pub mod wave_gen;
pub mod wave_gen_complex;
pub mod print;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DataKind {
    #[default]
    Empty,
    F64,
    ComplexF64,
}

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub kind: DataKind,

    pub f64_data: Option<Vec<f64>>,
    pub complex_f64: Option<Vec<num::Complex<f64>>>,
}

impl Into<Data> for Vec<f64>{
    fn into(self) -> Data {
        let mut data = Data::new(DataKind::F64);
        data.set_f64_data(self);
        data
    }
}

impl From<Data> for Vec<f64> {
    fn from(value: Data) -> Self {
        value.get_f64_data().clone()
    }
}

impl Into<Data> for Vec<()> {
    fn into(self) -> Data {
        Data::new(DataKind::Empty)
    }
}

impl From<Data> for Vec<()>{

    fn from(value: Data) -> Self {
        vec![]
    }
}

impl Data {
    pub fn new(kind: DataKind) -> Self {
        Self {
            kind,
            f64_data: None,
            complex_f64: None,
        }
    }

    pub fn set_f64_data(&mut self, data: Vec<f64>) {
        self.f64_data = Some(data);
    }

    pub fn set_complex_data(&mut self, data: Vec<num::Complex<f64>>) {
        self.complex_f64 = Some(data);
    }

    pub fn get_f64_data(&self) -> &Vec<f64> {
        self.f64_data.as_ref().expect("Data is not f64")
    }

    pub fn get_complex_data(&self) -> &Vec<num::Complex<f64>> {
        self.complex_f64.as_ref().expect("Data is not complex")
    }

    pub fn is_f64(&self) -> bool {
        self.kind == DataKind::F64
    }

    pub fn is_complex_f64(&self) -> bool {
        self.kind == DataKind::ComplexF64
    }

}

pub trait Stage<I,O>: Send + Sync {
    fn configure(&mut self, data: &mut PipelineSettings);
    fn process(&mut self, input: &Data, output: &mut Data);
    fn get_output_data_type(&self) -> DataKind;
    fn get_input_data_type(&self) -> DataKind;
}

pub trait FirstStage<I,O>: Stage<I,O>{
}

pub trait LastStage<I,O: Send + Sync>: Stage<I,O> {
}