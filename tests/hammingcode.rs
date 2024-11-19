// Test Triangle Window
use std::time::SystemTime;
use superdsp::ecc::hamming84::{self, hamming_decode};


#[test]
pub fn encode_decode() {
    for i in 0..=255{
        println!("Encoding {i}");
        let rval = hamming84::hamming_encode(i);
        let n1 = rval.0;
        let n2 = rval.1;
        //println!("{i} turns into {n1},{n2}.");
        //println!("Decoding {i}");
        let rval = hamming_decode((n1, n2));
        //println!("{n1} and {n2} turn into {x1} with the first {v1} and second {v2}.");
        assert_eq!((i,false,false), rval);
    }
}
#[test]
pub fn speed_test() {
    let start = SystemTime::now();
    let mut big_i: u64 = 0;
    while big_i < (10_u64.pow(6)){
        let i: u8 = (big_i%255) as u8;
        let r = hamming84::hamming_encode(i);
        hamming84::hamming_decode(r);
        big_i += 1;
    }
    match SystemTime::now().duration_since(start) {
        Ok(n) => println!("Process began {} milliseconds ago!", n.as_millis()),
        Err(_) => panic!("System traveled back in time!"),
    }
}
