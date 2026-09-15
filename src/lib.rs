pub fn bitboard() -> [u64; 13]{
    let mut bitboard = [0u64; 13];

    //Black
    bitboard[0] = 0b11111111 << 8;
    bitboard[1] = 0b10000001;
    bitboard[2] = 0b01000010;
    bitboard[3] = 0b00100100;
    bitboard[4] = 0b00010000;
    bitboard[5] = 0b00001000;

    //Black Rook & King flags
    bitboard[12] = 0b10001001;    

    //White
    bitboard[6] = 0b11111111 << 48;
    bitboard[7] = 0b10000001 << 56;
    bitboard[8] = 0b01000010 << 56;
    bitboard[9] = 0b00100100 << 56;
    bitboard[10] = 0b00001000 << 56;
    bitboard[11] = 0b00010000 << 56;

    //White Rook & King flags
    bitboard[12] |= 0b10010001 << 56;  
    return bitboard;
}

//If you bit shift a number over the threshold of being in the number, it just disappears. Nice.
//It uses u8s which does have some downsides, but also make it so much easier to bitshift over a border correctly.
const BITMASKS: [u8; 2] = [
    0b101 << 2, //Pawn + Knight 1
    //This is by default what black would see. To make it what white would see, bitshift -16. Also is used for knight stuff.
    0b10001 << 1, //Knight 2
];

//Needs a particular emphasis on testing
//Gives correct bitmasks in context to a given piece is. Mainly used for check.
pub fn assemble_bitmask(offset: (i32, i32), color: &Color) -> [u64; 11]{
    let mut board: [u64; 11] = [0; 11];
    let difference = ((offset.0 - 3), (offset.1 - 2));
    //pawn
    board[0] = shift_u8(BITMASKS[0], difference.0) as u64;
    if *color == Color::White {board[0] = board[0] << 16}
    //knight
    board[1] |= shift(shift_u8(BITMASKS[0], difference.0) as u64, difference.1 * 8);
    board[1] |= shift(shift_u8(BITMASKS[1], difference.0) as u64, 8 + difference.1 * 8);
    board[1] |= shift(shift_u8(BITMASKS[1], difference.0) as u64, 24 + difference.1 * 8);
    board[1] |= shift(shift_u8(BITMASKS[0], difference.0) as u64, 32 + difference.1 * 8);
    //king
    let temp = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
    for i in 0..8{
        board[2] |= shift(shift_u8(1, offset.0 + temp[i].0) as u64, (offset.1 + temp[i].1) * 8);
    }

    let mut lines: [u64; 2] = [0; 2];

    //vertical rays
    let temp = 7 - offset.1;
    for i in 0..8{
            if i < temp {
                lines[0] += 1 << 8 * i;
            } else {
                lines[1] += 1 << 8 * i;
            }
    }
    board[3] = lines[0] << offset.0;
    board[4] = lines[1] << offset.0;

    //horizontal rays
    board[5] = (shift_u8(0b11111111, 7 - offset.0) as u64) << offset.1 * 8;
    board[6] = (shift_u8(0b11111111, offset.0 - 7) as u64) << offset.1 * 8;

    //I'm mildly baffled at how long it took me to come up with this one segment of code.
    //diagonal rays
    for i in 1..8{
        board[7] |= shift(shift_u8(1, offset.0 + i) as u64, (offset.1 + i) * 8);
        board[8] |= shift(shift_u8(1, offset.0 - i) as u64, (offset.1 - i) * 8);
        board[9] |= shift(shift_u8(1, offset.0 - i) as u64, (offset.1 + i) * 8);
        board[10] |= shift(shift_u8(1, offset.0 + i) as u64, (offset.1 - i) * 8);
    }
    board
}

fn shift_u8(num: u8, offset: i32) -> u8 {
    if offset.abs() > 7 {return 0}
    if offset < 0 {
        return num >> offset.abs();
    } else {
        return num << offset;
    }
}

//This needs to be a thing since for some reason x << -1 != x >> 1. Weird.
pub fn shift(num: u64, offset: i32) -> u64 {
    if offset.abs() > 63 {return 0}
    if offset < 0 {
        return num >> offset.abs();
    } else {
        return num << offset;
    }
}

#[derive(PartialEq)]
pub struct Input {
    pub pos: u64,
    pub target: u64,
    pub piece: Piece,
    pub target_piece: Piece,
}

#[derive(PartialEq)]
pub enum Color {
    Black = 1,
    White = -1
}

#[derive(PartialEq, Debug)]
pub enum Piece{
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen
}

//Gives a bitboard of the current position and of the target, as well as identifies the type of piece it is.
pub fn input(raw_input: Vec<u8>, bitboard: &[u64; 13]) -> Option<Input>{
    for x in &raw_input{if *x > 8 {return None}}
    let pos = 1 << (raw_input[1] * 8) + raw_input[0];
    let target = 1 << (raw_input[3] * 8) + raw_input[2];

    let mut piece = Piece::None;
    let mut target_piece = Piece::None;
    for i in 0..12 {
        if bitboard[i] & pos != 0{
            piece = match i % 6{
                0 => {Piece::Pawn}
                1 => {Piece::Rook}
                2 => {Piece::Knight}
                3 => {Piece::Bishop}
                4 => {Piece::Queen}
                5 => {Piece::King}
                _ => {Piece::None}
            }
        }
    }

    for i in 0..12{
        if bitboard[i] & target != 0{
            target_piece = match i % 6{
                0 => {Piece::Pawn}
                1 => {Piece::Rook}
                2 => {Piece::Knight}
                3 => {Piece::Bishop}
                4 => {Piece::Queen}
                5 => {Piece::King}
                _ => {Piece::None}
            }
        }
    }
    Some(Input{pos: pos, target: target, piece: piece, target_piece: target_piece})
}

//This does the exact same job as shift, but in reverse. It does assume that there is only one number on the board, but it doesn't matter in context
pub fn find_coords(board: u64) -> (i32, i32){
    let zeros = board.trailing_zeros() as i32;
    let x = zeros % 8;
    let y = zeros / 8;
    return (x, y);
}

//while there is most certainly a better way to do this, I don't know it
pub fn boundary_check(x: i32) -> bool{
    if x <= 7 && x >= 0 {return true};
    false
}

