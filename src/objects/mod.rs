#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(feature = "gui")]
use std::thread::spawn;
#[cfg(feature = "std")]
use std::vec::Vec;

use spin::{Barrier, RwLock};

#[cfg(feature = "gui")]
use crate::gui::{DSPChart, GUI, Message};
use crate::objects::object::DSPObject;

pub mod object;
pub mod wave_gen;
pub mod viterbi_encode;
pub mod viterbi_decode;

#[cfg(feature = "std")]
pub mod wave_gen_time;

#[cfg(feature = "std")]
pub mod wave_gen_time_complex;
pub mod wave_gen_complex;
pub mod autocorrelation;

pub(crate) static F32_OUTPUT_BUFFERS: [RwLock<f32>; 64] = [
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
    RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0), RwLock::new(0.0),
];
pub(crate) static COMPLEX_OUTPUT_BUFFERS: [RwLock<num::Complex<f32>>; 64] = [
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
    RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)), RwLock::new(num::Complex::new(0.0, 0.0)),
];

#[cfg(feature = "multithreading-std")]
pub(crate) static BARRIERS: [Barrier; 64] = [
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
    Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5), Barrier::new(5),
];
pub(crate) static F32_OUTPUT_BUFFER_INDEX: spin::Mutex<usize> = spin::Mutex::new(0);
pub(crate) static COMPLEX_OUTPUT_BUFFER_INDEX: spin::Mutex<usize> = spin::Mutex::new(0);
pub(crate) static BARRIERS_INDEX: spin::Mutex<usize> = spin::Mutex::new(0);

#[cfg(feature = "gui")]
pub struct GUIExecutor {
    arr: Vec<*mut dyn DSPObject>,
    first: Box<dyn DSPObject>,
}

#[cfg(feature = "gui")]
impl GUIExecutor {
    pub fn run(arr: Vec<Box<dyn DSPChart<Message=Message, State=()>>>, mut first_element: Box<dyn DSPObject>) {
        spawn(move || {
            first_element.start()
        });

        let gui = GUI {
            width: 800,
            height: 600,
            elements: arr,
        };
        gui.start();
    }
}
