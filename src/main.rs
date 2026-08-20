use colored::Colorize;
use colored::ColoredString;
use std::io;

mod lib;
use lib::Input;
use lib::input;
use lib::bitboard;
use lib::Color;
fn main() {
    /*
    Order for pieces:
    Pawn, Rook, Knight, Bishop, Queen, King
    0-5 is black
    6-10 is white
    11 is e.p. and castling
     */
    let mut bitboard = bitboard();
    display_board(&bitboard);
}

fn player(bitboard: &mut [u64; 13], color: &Color){
    //this function gets player input and proccesses it to prove its validity, then changes the bitboard
    let input = get_input();
    while validate_input(bitboard, &input, color) {player(bitboard, color)}
    move_piece(bitboard, input);
}

fn validate_input(bitboard: &[u64; 13], input: &Input, color: &Color) -> bool{
    let mut temp = (false, false);
    for i in 0..12 {
        if bitboard[i] & input.pos != 0{temp.0 = true}
        if bitboard[i] & input.target != 0{temp.1 = true}
    }

    if temp.0 && temp.1 {
        
    }
    false
}

fn move_piece(bitboard: &mut [u64; 13], input: Input){

}

//This block of code is strictly for testing purposes. It will be removed after TUI is implemented.
fn get_input() -> Input {
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
fn display_board(bitboard: &[u64; 13]){
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
