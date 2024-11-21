use crate::objects::object::{Bus, DSPObject};
use crate::ecc::viterbi35::viterbi_decode;
use crate::objects::viterbi_encode::NUM_BYTES;

const RETURNED_BYTES: usize = NUM_BYTES;
const NUM_ENCODED_BYTES: usize = find_encoded(RETURNED_BYTES);


#[derive(Clone, Copy)]
pub struct ViterbiDecoder {
    pub input_bytes: Bus<'static>, 
    pub bus: Bus<'static>,
    pub bucket_array: [u8; NUM_ENCODED_BYTES],
    pub array_index: usize,
}

const fn find_encoded(NUM_BYTES_BASE: usize) -> usize{
    return NUM_BYTES_BASE*3;
}

impl ViterbiDecoder {
    pub fn new(input_bytes: Bus<'static>) -> ViterbiDecoder {
        let bus = Bus::new_f32();
        let bucket_array: [u8; NUM_ENCODED_BYTES] = [0; NUM_ENCODED_BYTES];
        let array_index = 0;

        ViterbiDecoder {
            input_bytes,
            bus,
            bucket_array,
            array_index,
        }
    }
}

impl DSPObject for ViterbiDecoder {
    fn return_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::F32
    }
    fn input_type(&self) -> crate::objects::object::Type {
        crate::objects::object::Type::NONE
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        &mut self.bus
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        self.bus = *bus;
    }

    fn process(&mut self){
        self.bucket_array[self.array_index] = *self.bus.buffer_f32.unwrap().read() as u8;
        self.array_index += 1;
        if self.array_index >= NUM_BYTES{
            let decoded_array = viterbi_decode::<NUM_ENCODED_BYTES, RETURNED_BYTES>(self.bucket_array);
            for value in decoded_array {
                self.bus.trigger_f32(value as f32);
            }
        }
    }

    fn start(&mut self) {
        loop {
            self.process();
        }
    }

}

