
//Takes one 8-bit byte and returns two Hamming(8,4)-coded SECDED bytes, at a rate of about 60MB/s.
pub fn hamming_encode(data: u8) -> (u8, u8){
    let mut data1 = data & 0xF0;
    let mut data2 = data << 4;
   //println!("Initial: Data 1 {data1}, data 2 {data2}");
    data1 |= (data1&0x10) >> 1; data1 &= 0xEF;
    data2 |= (data2&0x10) >> 1; data2 &= 0xEF;
    //println!("Bits moved: Data 1 {data1}, data 2 {data2}");
    let p1 = (data1 >> 3)&0x01 ^ (data1 >> 5)&0x01 ^ (data1 >> 7)&0x01; //Technically there's supposed to be another bit XOR'd in for each of these, but since that ends up being the p-bit itself it'll always be 0.
    let p2 = (data1 >> 3)&0x01 ^ (data1 >> 6)&0x01 ^ (data1 >> 7)&0x01;
    let p3 = (data1 >> 5)&0x01 ^ (data1 >> 6)&0x01 ^ (data1 >> 7)&0x01;
    data1 |= p1 << 1;
    data1 |= p2 << 2;
    data1 |= p3 << 4;
    let parity = (data1 >> 1)&0x01 ^ (data1 >> 2)&0x01 ^ (data1 >> 3)&0x01 ^ (data1 >> 4)&0x01 ^ (data1 >> 5)&0x01 ^ (data1 >> 6)&0x01 ^ (data1 >> 7)&0x01;
    data1 |= parity;
    let p1 = (data2 >> 3)&0x01 ^ (data2 >> 5)&0x01 ^ (data2 >> 7)&0x01;
    let p2 = (data2 >> 3)&0x01 ^ (data2 >> 6)&0x01 ^ (data2 >> 7)&0x01;
    let p3 = (data2 >> 5)&0x01 ^ (data2 >> 6)&0x01 ^ (data2 >> 7)&0x01;
    /*let x1 = (data2 >> 5)&0x01;
    let x2 = (data2 >> 6)&0x01;
    let x3 = (data2 >> 7)&0x01;
    println!("From {x1}, {x2}, and {x3} comes {p3}.");*/
    data2 |= p1 << 1;
    data2 |= p2 << 2;
    data2 |= p3 << 4;
    let parity = (data2 >> 1)&0x01 ^ (data2 >> 2)&0x01 ^ (data2 >> 3)&0x01 ^ (data2 >> 4)&0x01 ^ (data2 >> 5)&0x01 ^ (data2 >> 6)&0x01 ^ (data2 >> 7)&0x01;
    data2 |= parity;
    return (data1, data2);
}

//Takes two 8-bit Hamming(8,4)-coded bytes, and returns a at a rate of about 60MB/s.
pub fn hamming_decode(data: (u8, u8)) -> (u8, bool, bool){ //What a monstrous function - this is what happens when you go for speed over efficiency.
    let data1 = data.0;
    let data2 = data.1;
    let mut returned_byte: u8 = 0;
    let mut error1 = true;
    let mut error2 = true;
    let parity_found: bool = (data1&0x01) == 0x01; //I'm making an assumption that storing these as booleans now will save time later when comparing them.
    let parity_expected: bool = ((data1 >> 1)&0x01 ^ (data1 >> 2)&0x01 ^ (data1 >> 3)&0x01 ^ (data1 >> 4)&0x01 ^ (data1 >> 5)&0x01 ^ (data1 >> 6)&0x01 ^ (data1 >> 7)&0x01) == 0x01;
    //if parity_expected!=parity_found{println!("WRONG PARITY WITH VALUE {data1}")}
    let p1_found: bool = ((data1 >> 1)&0x01) == 0x01;
    let p2_found: bool = ((data1 >> 2)&0x01) == 0x01;
    let mut d1_found: bool = ((data1 >> 3)&0x01) == 0x01;
    let p3_found: bool = ((data1 >> 4)&0x01) == 0x01;
    let mut d2_found: bool = ((data1 >> 5)&0x01) == 0x01;
    let mut d3_found: bool = ((data1 >> 6)&0x01) == 0x01;
    let mut d4_found: bool = ((data1 >> 7)&0x01) == 0x01;
    //println!("Data found: D4 {d4_found} D3 {d3_found} D2 {d2_found} D1 {d1_found} P3 {p3_found} P2 {p2_found} P1 {p1_found} PP {parity_found}");
    let group1_valid: bool = !(d4_found ^ d2_found ^ d1_found ^ p1_found);
    let group2_valid: bool = !(d4_found ^ d3_found ^ d1_found ^ p2_found);
    let group3_valid: bool = !(d4_found ^ d3_found ^ d2_found ^ p3_found); 
    //println!("Groups found in byte 1: G1 {group1_valid} G2 {group2_valid} G3 {group3_valid}");
    //if !group3_valid{ println!("Group 3 invalid with value {data1}"); }
    match(group1_valid, group2_valid, group3_valid){
        (false, false, false) => d4_found = !d4_found,
        (true, false, false) => d3_found = !d3_found,
        (false, true, false) => d2_found = !d2_found,
        (false, false, true) => d1_found = !d1_found,
        _ => error1 = group1_valid&group2_valid&group3_valid
    } error1 = (parity_expected==parity_found)^error1; //If total parity was wrong but there's no hamming error, we have a double-bit error and need the byte to be retransmitted!
    returned_byte += ((d4_found as u8) << 7)+((d3_found as u8) << 6)+((d2_found as u8) << 5)+((d1_found as u8) << 4);
    let parity_found: bool = (data2&0x01) == 0x01; //I'm making an assumption that storing these as booleans now will save time later when comparing them.
    let parity_expected: bool = ((data2 >> 1)&0x01 ^ (data2 >> 2)&0x01 ^ (data2 >> 3)&0x01 ^ (data2 >> 4)&0x01 ^ (data2 >> 5)&0x01 ^ (data2 >> 6)&0x01 ^ (data2 >> 7)&0x01) == 0x01;
    //if parity_expected!=parity_found{println!("WRONG PARITY WITH VALUE {data2}")}
    let p1_found: bool = ((data2 >> 1)&0x01) == 0x01;
    let p2_found: bool = ((data2 >> 2)&0x01) == 0x01;
    let mut d1_found: bool = ((data2 >> 3)&0x01) == 0x01;
    let p3_found: bool = ((data2 >> 4)&0x01) == 0x01;
    let mut d2_found: bool = ((data2 >> 5)&0x01) == 0x01;
    let mut d3_found: bool = ((data2 >> 6)&0x01) == 0x01;
    let mut d4_found: bool = ((data2 >> 7)&0x01) == 0x01;
    //println!("Data found: D4 {d4_found} D3 {d3_found} D2 {d2_found} D1 {d1_found} P3 {p3_found} P2 {p2_found} P1 {p1_found} PP {parity_found}");
    let group1_valid: bool = !(d4_found ^ d2_found ^ d1_found ^ p1_found);
    let group2_valid: bool = !(d4_found ^ d3_found ^ d1_found ^ p2_found);
    let group3_valid: bool = !(d4_found ^ d3_found ^ d2_found ^ p3_found);
    //println!("Groups found in byte 2: G1 {group1_valid} G2 {group2_valid} G3 {group3_valid}");
    //if !group3_valid{ println!("Group 3 invalid with value {data2}"); }
    match(group1_valid, group2_valid, group3_valid){
        (false, false, false) => d4_found = !d4_found,
        (true, false, false) => d3_found = !d3_found,
        (false, true, false) => d2_found = !d2_found,
        (false, false, true) => d1_found = !d1_found,
        _ => error2 = group1_valid&group2_valid&group3_valid
    } error2 = (parity_expected==parity_found)^error2;
    returned_byte += ((d4_found as u8) << 3)+((d3_found as u8) << 2)+((d2_found as u8) << 1)+((d1_found as u8));
    return (returned_byte, error1, error2);
}