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
    //We do need to apply it to all Rooks and Kings.
    return bitboard;
}

//This needs to be a thing since for some reason x << -1 != x >> 1. Weird.
pub fn shift(num: u64, offset: i32) -> u64 {
    if offset < 0 {
        return (num >> offset.abs());
    } else {
        return (num << offset);
    }
}

#[derive(PartialEq)]
pub struct Input {
    pub pos: u64,
    pub target: u64,
}

#[derive(PartialEq)]
pub enum Color {
    Black = 1,
    White = -1
}

pub fn input(raw_input: Vec<u8>) -> Option<Input>{
    for x in &raw_input{if *x > 8 {return None}}
    let pos = 1 << (raw_input[1] * 8) + raw_input[0];
    let target = 1 << (raw_input[3] * 8) + raw_input[2];
    Some(Input{pos: pos, target: target})
}

//This does the exact same job as shift, but in reverse. It does assume that there is only one number on the board, but it doesn't matter in context
pub fn find_coords(board: u64) -> (u8, u8){
    let mut board = board;
    if board == 0 {return(0, 0)}
    if board == 1 {return(1, 0)}
    for i in 0..65{
        board = board >> 1;  
        if board == 1 {return((i % 8), (i / 8))};  
    }
    (0, 0)
}


