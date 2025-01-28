use rustdsp::pipeline::{PipelineBuilder, PipelineSettings};
use rustdsp::pipeline::PipelineType::Loop;
use rustdsp::prelude::Pipeline;
use rustdsp::prelude::print::Print;
use rustdsp::prelude::wave_gen::WaveGen;

#[tokio::main]
async fn main() {
    let mut settings = PipelineSettings::default();
    
    settings.set_frequency(1.0)
        .set_sample_rate(2.0)
        .set_sps(1)
        .set_num_taps(1);
    
    let pipeline: Pipeline<Vec<()>,Vec<()>> = PipelineBuilder::default()
        .add_first_stage(Box::new(WaveGen::new()))
        .add_last_stage(Box::new(Print::default()))
        .build(Loop, &mut settings);

    loop {
        
    }
}