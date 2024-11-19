use std::time::SystemTime;
use superdsp::ecc::viterbi35::{self, viterbi_decode, viterbi_encode};


#[test]
pub fn encode_decode() {
    let viterbi = viterbi35::viterbi_encode::<16, 18, 54>([75, 0x01, 0x21, 0x23, 0x43, 0x45, 0x65, 0x67, 0x87, 0x89, 0xA9, 0xAB, 0xCB, 0xCD, 0xED, 0xEF]);
    print!("Encoded bytes: ");
    for byte in viterbi{
        /*for i in 0..=7{
            let bit = (byte>>i)&0x01;
            //println!("{bit}");
        }*/
        print!("{byte}, ");
    }
    assert_eq!(viterbi, [0, 0, 84, 244, 215, 251, 3, 0, 244, 3, 250, 85, 28, 250, 85, 28, 208, 251, 254, 208, 251, 254, 42, 90, 225, 42, 90, 225, 128, 138, 235, 135, 138, 235, 125, 43, 244, 125, 43, 244, 87, 133, 22, 87, 133, 22, 173, 36, 9, 173, 112, 0, 0, 0]);
    let mut corrupted_encoded = viterbi;
    for i in 0..=14{
        if i%2 == 0 {corrupted_encoded[i] = viterbi[i] ^ (0x01 << (i%8));} //Flip "random" (not really) bits.
        /*let that_num = corrupted_encoded[i];
        let this_num = encoded_array[i];
        println!("{this_num}, {that_num}");*/
    }
    //let decoded: [u8; 16] = viterbi_decode::<54, 16>(viterbi);
    let decrypted: [u8; 16] = viterbi_decode::<54, 16>(corrupted_encoded);
    println!("");
    /*print!("Decoded bytes: ");
    for byte in decoded{
        for i in 0..=7{
            let bit = (byte>>i)&0x01;
            //println!("{bit}");
        }
        print!("{byte}, ");
    }*/
    println!("");
    print!("Decrypted bytes: ");
    for byte in decrypted{
        /*for i in 0..=7{
            let bit = (byte>>i)&0x01;
            //println!("{bit}");
        }*/
        print!("{byte}, ");
    }
    assert_eq!(decrypted, [75, 0x01, 0x21, 0x23, 0x43, 0x45, 0x65, 0x67, 0x87, 0x89, 0xA9, 0xAB, 0xCB, 0xCD, 0xED, 0xEF]);
}
#[test]
pub fn small_bytes(){
    let small_byte_array = [0x0, 0b01010101, 0xFF];
    let encoded_array = viterbi35::viterbi_encode::<3, 5, 15>(small_byte_array);
    let mut corrupted_encoded = encoded_array;
    for i in 0..=14{
        if i%2 == 0{corrupted_encoded[i] = encoded_array[i] ^ (0x01 << (i%8));} //Flip "random" (not really) bits.
        let that_num = corrupted_encoded[i];
        let this_num = encoded_array[i];
        println!("{this_num}, {that_num}");
    }
    let mut k = 0;
    for i in corrupted_encoded{
        for d in 0..7{
            let bit = (i >> d)%2;
            print!("{bit}");
            if k%3==2{
                print!(" ");
            } k+=1;
        }
    } k=0; println!();
    for i in encoded_array{
        for d in 0..7{
            let bit = (i >> d)%2;
            print!("{bit}");
            if k%3==2{
                print!(" ");
            } k+=1;
        }
    } println!();
    //let _good = viterbi_decode::<15, 3>(encoded_array);
    let bad = viterbi_decode::<15, 3>(corrupted_encoded);
    for byte in bad{
        println!("{byte} and");
    }
    
}
#[test]

pub fn every_byte() {
    let mut bytes_array: [u8; 256] = [0; 256];
    for i in 0..=255{
        bytes_array[i] = i as u8;
    }
    let encoded_array = viterbi35::viterbi_encode::<256, 258, 774>(bytes_array);
    let mut corrupted_encoded = encoded_array;
    for i in 0..=773{
        corrupted_encoded[i] = encoded_array[i] ^ (0x40 << (i%8)); //Flip "random" (not really) bits.
        let that_num = corrupted_encoded[i];
        let this_num = encoded_array[i];
        println!("{this_num}, {that_num}");
    }
    
    let good = viterbi_decode::<774, 256>(encoded_array);
    let bad = viterbi_decode::<774, 256>(corrupted_encoded);
    for byte in good{
        /*for i in 0..=7{
            let bit = (byte>>i)&0x01;
            //println!("{bit}");
        }*/
        print!("{byte}, ");
    }
    println!();
    println!();
    println!();
    for byte in bad{
        /*for i in 0..=7{
            //let bit = (byte>>i)&0x01;
            //println!("{bit}");
        }*/
        print!("{byte}, ");
    }
    assert_eq!(good, bad);
}
#[test]
pub fn speed_test() {
    let mut array = [0; 1000];
    let mut big_i: i32 = 0;
    let great: i32 = 10000000;
    while big_i < (10_i32.pow(3)){
        let i: u8 = (big_i%255) as u8;
        array[big_i as usize] = i;
        big_i += 1;
    }
    big_i = 0;
    let mut start = SystemTime::now();
    while big_i < (great/10_i32.pow(3)){
        let encoded = viterbi_encode::<1000, 1002, 3006>(array);
        let _decoded = viterbi_decode::<3006, 1000>(encoded);
        big_i += 1;
    }
    big_i = 0;
    let mut total_time = 0;
    match SystemTime::now().duration_since(start) {
        Ok(n) => {total_time = n.as_nanos()/great as u128;},
        Err(_) => panic!("System traveled back in time!"),
    }
    start = SystemTime::now();
    while big_i < (great/10_i32.pow(3)){
        let _encoded = viterbi_encode::<1000, 1002, 3006>(array);
        big_i += 1;
    }
    let mut encoding_time = 0;
    match SystemTime::now().duration_since(start) {
        Ok(n) => {encoding_time = n.as_nanos()/great as u128;},
        Err(_) => panic!("System traveled back in time!"),
    }
    let decoding_time = total_time-encoding_time;
    println!("encoding time per byte ...  {encoding_time} nanoseconds");
    println!("decoding time per byte ...  {decoding_time} nanoseconds");
    println!("total time per byte ...  {total_time} nanoseconds");
}

