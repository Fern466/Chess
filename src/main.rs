use colored::Colorize;
use colored::ColoredString;
use std::io;
fn main() {
    /*
    Order for pieces:
    Pawn, Rook, Knight, Bishop, Queen, King
    0-5 is black
    6-10 is white
    11 is e.p. and castling
     */
    let mut bitboard = bitboard();
    let input = get_input();
    display_board(bitboard);
}

//This block of code is strictly for testing purposes. It will be removed after TUI is implemented.
fn get_input() -> input {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let temp = temp.trim();
    let mut s: Vec<u8> = vec![];
        for i in 0..temp.len(){
        let num = temp[i..i + 1].parse::<char>();
        if !(num.is_err()) {
            let x = num.unwrap().to_digit(18);
            if x != None{
                let mut x = x.unwrap();
                if x != 9 || x != 0 {
                    if x > 9 {
                        x = x % 9;
                    }
                    s.push(x.try_into().unwrap());
                }
            }
        }
    }

    let input = input(s);
    if input == None {get_input()} else {return input.unwrap()}
}

//Again, temporary.
fn display_board(bitboard: [u64; 13]){
    //can't put these as constants for some reason
    let pieces: [ColoredString; 12] = ["P".black(), "R".black(), "K".black(), "B".black(), "Q".black(), "$".black(), "P".white(), "R".white(), "K".white(), "B".white(), "Q".white(), "$".white()];

    for y in 0..8{
        for x in 0..8 {
            let mut letter = ".".black();
            for i in 0..12{
                if 1 << (y * 8) + x & bitboard[i] != 0{
                    letter = pieces[i].clone();
                }
            }
            print!("{letter}")
        }
        println!("\n");
    }
}

fn bitboard() -> [u64; 13]{
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
pub struct input {
    pos: u64,
    target: u64,
}


pub fn input(raw_input: Vec<u8>) -> Option<input>{
    for x in &raw_input{if *x > 7 || *x < 0 {return None}}
    let pos = 1 << (raw_input[1] * 8) + raw_input[0];
    let target = 1 << (raw_input[3] * 8) + raw_input[2];
    Some(input{pos: pos, target: target})
}


