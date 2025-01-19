pub mod wave_gen;
pub mod wave_gen_complex;

pub trait Stage {
    type Input;
    type Output;
    async fn process(&mut self, input: Self::Input) -> Self::Output;
}