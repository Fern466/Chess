use colored::Colorize;
use colored::ColoredString;
use std::io;

//is there a way to import the whole file?
mod lib;
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
}

fn player(bitboard: &mut [u64; 13], color: &Color){
    //this function gets player input and proccesses it to prove its validity, then changes the bitboard
    let input = get_input();
    while validate_input(bitboard, &input, color) {player(bitboard, color)}
    move_piece(bitboard, input);
}

fn validate_input(bitboard: &mut [u64; 13], input: &Input, color: &Color) -> bool{
    //first will be friendlies, last enemies
    let mut boards = (0, 0);
    for i in 0..12{
        if i < 6 && *color == Color::Black{boards.0 &= bitboard[i]} else {boards.1 &= bitboard[i]}
        if i > 5 && *color == Color::White{boards.0 &= bitboard[i]} else {boards.1 &= bitboard[i]}
    }

    //cleaning up any en passant opportunities. We don't need to check anything for it here since an en passant would have happened by the time it is the players turn
    if *color == Color::Black {bitboard[13] |= (0b00000000 << 8)} else {bitboard[13] |= (0b00000000 << 48)}

    //This if statement is just a "gatekeeper" since otherwise we'd need to check this later
    if boards.0 & input.target == 0{
        for i in 0..12 {
            if bitboard[i] & input.pos != 0{
                match i % 6{
                    0 => {return pawn(bitboard, input, color, &boards.1)}
                    1 => {}
                    2 => {}
                    3 => {}
                    4 => {
                        if *color == Color::Black {} else {}
                    }
                    5 => {
                        if *color == Color::Black {} else {}
                    }
                    _ => {}
                }
            }
        }
    } else {
        //castling goes here
    }
    false
}

fn pawn(bitboard: &mut [u64; 13], input: &Input, color: &Color, enemy_board: &u64) -> bool{
    let offset = if *color == Color::Black {1} else {-1};
    if shift(input.pos, offset * 8) == input.target {return true}
    //Beware, this does interact with en passant code
    let temp = find_coords(input.pos);
    let correct_pos = (temp.0 == 1 || *color == Color::Black) || (temp.0 == 6 || *color == Color::White);
    if shift(input.pos, offset * 16) == input.target && correct_pos {
        bitboard[13] |= input.target;
        return true;
    }
    if shift(input.pos, (offset * 8) + 1) == input.target && enemy_board & input.target != 0 {return true}
    if shift(input.pos, (offset * 8) - 1) == input.target && enemy_board & input.target != 0 {return true}
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
