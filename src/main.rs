use colored::Colorize;
use colored::ColoredString;
use std::io;

//is there a way to import the whole file?
mod lib;
use lib::Piece;
use lib::Input;
use lib::input;
use lib::bitboard;
use lib::Color;
use lib::shift;
use lib::find_coords;
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
    player(&mut bitboard, &Color::White);
    display_board(&bitboard);
}

fn player(bitboard: &mut [u64; 13], color: &Color){
    //this function gets player input and proccesses it to prove its validity, then changes the bitboard
    let input = get_input(bitboard);
    println!("{:?}", input.piece);
    while !validate_input(bitboard, &input, color) {player(bitboard, color)}
    move_piece(bitboard, input, color);
}

fn validate_input(bitboard: &mut [u64; 13], input: &Input, color: &Color) -> bool{
    //first will be friendlies, last enemies
    let mut boards = (0, 0);
    for i in 0..12{
        if i < 6 && *color == Color::Black{boards.0 |= bitboard[i]} else {boards.1 |= bitboard[i]}
        if i > 5 && *color == Color::White{boards.0 |= bitboard[i]} else {boards.1 |= bitboard[i]}
    }


    //This if statement is just a "gatekeeper" since otherwise we'd need to check this later
    if boards.0 & input.target == 0{
        match input.piece {
            Piece::Pawn => {return pawn(bitboard, input, color, &boards.1)}
            Piece::Rook => {}
            Piece::Knight => {}
            Piece::Bishop => {}
            Piece::Queen => {}
            Piece::King => {}
            _ => {return false}
        }
    } else {
        //castling goes here
    }
    false
}

fn pawn(bitboard: &mut [u64; 13], input: &Input, color: &Color, enemy_board: &u64) -> bool{
    //This technically doesn't have a purpose yet. It should be after e.p. code and after double move code.
    clean_bitboard(bitboard);

    //Beware, this does interact with en passant code
    let offset = if *color == Color::Black {1} else {-1};
    let temp = find_coords(input.pos);
    let correct_pos: bool = (temp.0 == 1 || *color == Color::Black) || (temp.0 == 6 || *color == Color::White);
    if shift(input.pos, offset * 16) == input.target && correct_pos && (enemy_board & input.target) == 0 {
        bitboard[12] |= input.target;
        return true;
    }    
    if shift(input.pos, offset * 8) == input.target && (enemy_board & input.target) == 0 {return true}
    if shift(input.pos, (offset * 8) + 1) == input.target && (enemy_board & input.target) != 0 {return true}
    if shift(input.pos, (offset * 8) - 1) == input.target && (enemy_board & input.target) != 0 {return true}
    false
}


fn move_piece(bitboard: &mut [u64; 13], input: Input, color: &Color){
    let mut i = if *color == Color::White{6} else {0};
    i += match input.piece{
            Piece::Pawn => {0}
            Piece::Rook => {1}
            Piece::Knight => {2}
            Piece::Bishop => {3}
            Piece::Queen => {4}
            Piece::King => {5}
            _ => {12}
    };

    bitboard[i] ^= input.pos;

    for j in 0..12{
        if bitboard[j] & input.target != 0{
            bitboard[j] |= input.target;
        }
    }

    bitboard[i] |= input.target;

    //This is in case there is a special state that needs to be taken care of (aka of the 13th board)
    if bitboard[12] & input.pos != 0 {
        bitboard[12] |= input.pos;
        if bitboard[12] & input.target == 0 {
            bitboard[12] |= input.target;
        }
    }
}

//Cleans the pawn part of the 13th board
fn clean_bitboard(bitboard: &mut [u64; 13]){
    for i in 0..5{
        let temp = 0b11111111 << 24 + (8 * i);
        bitboard[12] &= !temp;
    }
}

//This block of code is strictly for testing purposes. It will be removed after TUI is implemented.
fn get_input(bitboard: &[u64; 13]) -> Input {
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

    let input = input(s, bitboard);
    if input == None {get_input(bitboard)} else {return input.unwrap()}
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
