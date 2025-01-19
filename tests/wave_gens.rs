use rustdsp::stages::Stage;
use rustdsp::stages::wave_gen::WaveGen;

#[tokio::test]
pub async fn wave_gen() {
    let mut wave_gen = WaveGen::new(, 1.0, 1.0, 1.0);
    let mut output = Vec::new();
    for _ in 0..10 {
        output.push(wave_gen.process(()).await);
    }
    assert_eq!(output, vec![1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0]);
}