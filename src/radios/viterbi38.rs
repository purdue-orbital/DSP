//use std::{print, println};

pub fn viterbi_encode<const NUM_BYTES: usize, const NUM_BYTES_WITH_PADDING: usize, const RETURNED_BYTES: usize>(data_original: [u8; NUM_BYTES]) -> [u8; RETURNED_BYTES]{
    let mut final_array = [0 as u8; RETURNED_BYTES];
    let mut data = [0 as u8; NUM_BYTES_WITH_PADDING];
    for i in 0..NUM_BYTES{
        data[i+1] = data_original[i];
        //let val = data[i];
        //println!("Data is {val}");
    } //This is not the fastest way to do this. The fastest way, however, is very ugly and also doesn't work. This is also ugly, but less so, and it does work.
    /*For anyone interested who may wish to implement this in the future, I originally intended to add the padding bytes by calculating them inline during the function execution, 
    (just doing a few loops with current_tuple values hardcoded to 0 at the start and ends) thus removing the need for this loop and the constant parameter. 
    It was, however, horrifically spaghetti, and also it wouldn't work for some reason, and because of its spaghettiness was very hard to debug.
    If someone else can get that to work, though, it will be slightly faster than this.
    */
    let mut index: u32 = 0;
    let mut outdex: u32 = 0;
    while index < (NUM_BYTES_WITH_PADDING as u32*8)-3 {
        let current_tuple = 
            ((((data[index as usize/8])>>(index%8))&0x01),
            (((data[(index as usize+1)/8])>>(index+1)%8)&0x01),
            (((data[(index as usize+2)/8])>>(index+2)%8)&0x01)); //It's true! This goes through the byte backwards. It's faster that way.
        //let one = current_tuple.0; let two = current_tuple.1; let three = current_tuple.2; println!("Current tuple is {one}, {two}, {three} at id {index}");
        index += 1;
        let processed_tuple = 
            (((current_tuple.0^current_tuple.1^current_tuple.2) << outdex%8), //Why doesn't the just use a look up table? Is he Stupid?
            ((current_tuple.0^current_tuple.1) << ((outdex+1)%8)),
            ((current_tuple.1^current_tuple.2) << ((outdex+2)%8)));
        //let one = processed_tuple.0; let two = processed_tuple.1; let three = processed_tuple.2; println!("XOR'd tuple is {one}, {two}, {three}");
        final_array[(outdex as usize)/8] |= processed_tuple.0; outdex += 1;
        final_array[(outdex as usize)/8] |= processed_tuple.1; outdex += 1;
        final_array[(outdex as usize)/8] |= processed_tuple.2; outdex += 1;
    }
    return final_array;
}
pub fn viterbi_decode<const NUM_BYTES: usize, const RETURNED_BYTES: usize>(data: [u8; NUM_BYTES]) -> [u8; RETURNED_BYTES]{
    let valids = [[0, 5], [6, 3], [1, 4], [7, 2], [1, 4], [7, 2], [0, 5], [6, 3]];
    let mut final_array = [0; RETURNED_BYTES];
    let mut last_value: u8 = 0;
    let mut index: u32 = 18;
    let mut outdex: u32 = 0;
    while index < (NUM_BYTES*8-30) as u32{
        let current_tuple = 
            ((((data[index as usize/8])>>(index%8))&0x01),
            (((data[(index as usize+1)/8])>>(index+1)%8)&0x01),
            (((data[(index as usize+2)/8])>>(index+2)%8)&0x01));
        let true_value: u8 = (current_tuple.0<<2) | (current_tuple.1<<1) | (current_tuple.2);
        //println!("I read {true_value} at bit {real_index}.");
        let zero = valids[last_value as usize][0] == true_value;
        let mut one = valids[last_value as usize][1] == true_value;
        if !(zero | one){ //If neither the 0 nor the 1 account for this read word - we have an error!
            //println!("That's an error, the last was {last_value}!");
            /*let zero_value = valids[last_value as usize][0];
            let one_value = valids[last_value as usize][1];
            let zero_tuple = (zero_value>>2,(zero_value>>1)%2,(zero_value>>2)%2);
            let one_tuple = (one_value>>2,(one_value>>1)%2,(one_value>>2)%2);
            let zero_score = (zero_tuple.0==current_tuple.0) as u8 + (zero_tuple.1==current_tuple.1) as u8 + (zero_tuple.2==current_tuple.2) as u8;
            let one_score = (one_tuple.0==current_tuple.0) as u8 + (one_tuple.1==current_tuple.1) as u8 + (one_tuple.2==current_tuple.2) as u8;
            println!("Zero score is {zero_score}");
            println!("One score is {one_score}");
            one = zero_score < one_score;
            println!("Using one? {one}");*/
            let preserved_last_value = last_value;
            let mut paths_array: [u8; 256] = [0; 256];
            let mut record_lowest: u8 = 6;
            let mut record_lowest_entry: u8 = 0;
            for entry in 0..32{
                let mut truncated_entry_no = entry;
                let mut index_offset: u32 = 0;
                while (index_offset < 5) && index+index_offset<(NUM_BYTES*8-30) as u32{
                    let index = index + index_offset*3;
                    let current_tuple = 
                        ((((data[index as usize/8])>>(index%8))&0x01),
                        (((data[(index as usize+1)/8])>>(index+1)%8)&0x01),
                        (((data[(index as usize+2)/8])>>(index+2)%8)&0x01));
                    let true_value: u8 = (current_tuple.0<<2) | (current_tuple.1<<1) | (current_tuple.2);
                    let current_bit = truncated_entry_no%2;
                    let expected_value = valids[last_value as usize][current_bit];
                    //print!("E{expected_value}-R{true_value}-");
                    let is_error =  expected_value != true_value;
                    //println!("Pattern {entry}. The last value was {last_value}, and the current bit is {current_bit}. I expected {expected_value} and found {true_value}.");
                    if is_error {
                        paths_array[entry] += 1;
                    }
                    last_value = expected_value;
                    index_offset += 1;
                    truncated_entry_no >>= 1;
                } //println!();
                let result = paths_array[entry];
                if result<record_lowest {
                    record_lowest = result;
                    record_lowest_entry = entry as u8;
                }
                //print!("Pattern no. {entry} has {result} errors. ");
                last_value = preserved_last_value;
            } //println!();
            let decided_bit = (record_lowest_entry)%2;
            one = decided_bit != 0;
            if index != 0 {let previous_tuple = 
            ((((data[(index-3) as usize/8])>>((index-3)%8))&0x01),
            (((data[(index-2) as usize/8])>>(index-2)%8)&0x01),
            (((data[((index-1) as usize)/8])>>(index-1)%8)&0x01));
            let previous_value: u8 = (previous_tuple.0<<2) | (previous_tuple.1<<1) | (previous_tuple.2);
            last_value = valids[previous_value as usize][decided_bit as usize];
            //println!("Record lowest entry was pattern no. {record_lowest_entry}, so we use {decided_bit} and set last value to {last_value}.");
        }
        } else { last_value = true_value%8; }
        final_array[(outdex as usize)/8] |= (one as u8)<<(outdex%8); outdex += 1;
        index += 3;
    }
    return final_array;
}
