use colored::Colorize;
use colored::ColoredString;
use std::io;
fn main() {
    /*
    Order for pieces:
    Pawn, Rook, Knight, Bishop, Queen, King
    0-5 is white
    6-10 is black
    11 is e.p. and castling
     */
    let mut bitboard = bitboard();
    display_board(bitboard);
}

//This block of code is strictly for testing purposes. It will be removed after TUI is implemented.
fn get_input() -> (input, input) {
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

    let pos = input {x: s[0], y: s[1]};
    let target = input {x: s[2], y: s[3]};

    (pos, target)
}

//Again, temporary.
fn display_board(bitboard: [u64; 13]){
    //can't put these as constants for some reason
    let pieces: [ColoredString; 12] = ["P".white(), "R".white(), "K".white(), "B".white(), "Q".white(), "$".white(), "P".black(), "R".black(), "K".black(), "B".black(), "Q".black(), "$".black()];
    let mut all_pieces: u64 = 0;
    for i in 0..12{
        all_pieces = all_pieces | bitboard[i];
    }

    for y in 0..8{
        for x in 0..8 {
            let mut letter = ".".black();
            for i in 0..12{
                if 1 << (y * 8) + x & bitboard[i] == 1{
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

    //White
    bitboard[0] = 0b11111111 << 8;
    bitboard[1] = 0b10000001;
    bitboard[2] = 0b01000010;
    bitboard[3] = 0b00100100;
    bitboard[4] = 0b00010000;
    bitboard[5] = 0b00001000;

    //Black
    bitboard[6] = 0b11111111 << 48;
    bitboard[7] = 0b10000001 << 56;
    bitboard[8] = 0b01000010 << 56;
    bitboard[9] = 0b00100100 << 56;
    bitboard[10] = 0b00001000 << 56;
    bitboard[11] = 0b00010000 << 56;

    //The last piece of the board doesn't need to be modified since it only gets added to in special circumstances.
    return bitboard;
}

pub struct input {
    x: u8,
    y: u8,
}

impl input {
    pub fn input(x: u8, y: u8) -> Option<Self>{
        if x < 8 && y < 8 {
            return Some(input{x,y});
        } else {
            None
        }
    }
}
