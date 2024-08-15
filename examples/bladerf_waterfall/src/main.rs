use rustdsp::gui::time_chart::TimeChart;
use rustdsp::objects::Pipeline;
use rustdsp::radios;

fn main() {
    let mut pipeline = Pipeline::new();
    
    let mut radio = radios::bladerf::BladeRF::new(916000000, 100000, 20, 200000, 1024);
    let mut chart = TimeChart::new();

    pipeline.add_object(&mut radio);
    pipeline.add_gui_element(&mut chart);

    pipeline.process();
}