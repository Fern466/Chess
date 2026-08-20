pub fn bitboard() -> [u64; 13]{
    let mut bitboard = [0u64; 13];

    //Black
    bitboard[0] = 0b11111111 << 8;
    bitboard[1] = 0b10000001;
    bitboard[2] = 0b01000010;
    bitboard[3] = 0b00100100;
    bitboard[4] = 0b00010000;
    bitboard[5] = 0b00001000;

    

    //White
    bitboard[6] = 0b11111111 << 48;
    bitboard[7] = 0b10000001 << 56;
    bitboard[8] = 0b01000010 << 56;
    bitboard[9] = 0b00100100 << 56;
    bitboard[10] = 0b00001000 << 56;
    bitboard[11] = 0b00010000 << 56;

    //The last piece of the board doesn't need to be modified since it only gets added to in special circumstances.
    return bitboard;
}

#[derive(PartialEq)]
pub struct Input {
    pub pos: u64,
    pub target: u64,
}

pub enum Color {
    Black,
    White
}


pub fn input(raw_input: Vec<u8>) -> Option<Input>{
    for x in &raw_input{if *x > 8 {return None}}
    let pos = 1 << (raw_input[1] * 8) + raw_input[0];
    let target = 1 << (raw_input[3] * 8) + raw_input[2];
    Some(Input{pos: pos, target: target})
}


