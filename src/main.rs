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
use lib::boundary_check;
fn main() {
    /*
    Order for pieces:
    Pawn, Rook, Knight, Bishop, Queen, King
    0-5 is black
    6-10 is white
    11 is e.p. and castling
     */
    let mut bitboard = bitboard();

    bitboard[8] &= !(0b11111111 << 56);
    bitboard[9] &= !(0b11111111 << 56);
    bitboard[10] &= !(0b11111111 << 56);

    loop{
        display_board(&bitboard);
        player(&mut bitboard, &Color::White);
    }
}

fn player(bitboard: &mut [u64; 13], color: &Color){
    //this function gets player input and proccesses it to prove its validity, then changes the bitboard
    let input = get_input(bitboard);
    println!("{:?}", input.piece);
    if validate_input(bitboard, &input, color) {
        move_piece(bitboard, input, color);
    } else {player(bitboard, color)}
}

const KING: [(i32, i32); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
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
            Piece::Knight => {
                clean_bitboard(bitboard);
                return knight(input)
            }
            Piece::Queen | Piece::Bishop | Piece::Rook => {
                //rook should  have castling
                clean_bitboard(bitboard);
                return diagonal_and_straight_check(input, boards.0 | boards.1);
            }
            Piece::King => {
                //King does have to have castling.
                clean_bitboard(bitboard);
                let offset = find_coords(input.pos);
                let target = find_coords(input.target);
                for i in 0..8{
                    if (offset.0 + KING[i].0, offset.1 + KING[i].1) == target {
                        return true;
                    }
                }
            }
            _ => {return false}
        }
    } else {
        if input.piece == Piece::King && input.target_piece == Piece::Rook{
            return castle(bitboard, input, boards.0 | boards.1);
        }
    }
    false
}

fn pawn(bitboard: &mut [u64; 13], input: &Input, color: &Color, enemy_board: &u64) -> bool{
    let offset = if *color == Color::Black {1} else {-1};
    //en passant. I do need to figure out how to test this exactly
    let left_ep = shift(input.pos, (offset * 8) - 1) == input.target && shift(input.pos, (offset * 8) - 1) & bitboard[12] != 0;
    let right_ep = shift(input.pos, (offset * 8) + 1) == input.target && shift(input.pos, (offset * 8) + 1) & bitboard[12] != 0;
    if (left_ep || right_ep ) && input.target_piece == Piece::None{return true}
    
    clean_bitboard(bitboard);

    
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

const KNIGHT_TRANSFORM: [(i32, i32); 8] = [(2, 1), (1, 2), (2, -1), (1 , -2), (-2, 1), (-1, -2), (-2, 1), (2, -1)];
fn knight(input: &Input) -> bool{
    //this converts the bitboards to an x, y format since it becomes much easier to check them
    for i in 0..8{
        let offset = find_coords(input.pos);
        let target = find_coords(input.target);
        let target = (target.0 as i32, target.1 as i32);
        let knight: (i32, i32) = (offset.0 as i32 + KNIGHT_TRANSFORM[i].0, offset.1 as i32 + KNIGHT_TRANSFORM[i].1);
        if knight == target {return true};
    }
    false
}

//board in this situation is the union of the first 12 boards. We already have checking for friendlies outside of this, so it should just function the same. 
//We NEED to check all the board though
fn diagonal_and_straight_check(input: &Input, board: u64) -> bool{
    //yoinked a lot of the code from chess_old since its suprisingly good. Also made a lot of new improvements
    let offset = find_coords(input.pos);
    let target = find_coords(input.target);
    let displacement = ((target.0 - offset.0).signum(), (target.1 - offset.1).signum());
    let mut obstructed = false;

    //makes sure that those pieces can only do the appproved movement
    if input.piece == Piece::Rook{if displacement.0 != 0 && displacement.1 != 0 {return false}}
    if input.piece == Piece::Bishop{if displacement.0 == 0 || displacement.1 == 0 {return false}}

    for i in 1..8{
        if obstructed{return false}
        let pos = (offset.0 + displacement.0 * i, offset.1 + displacement.1 * i);
        if !boundary_check(pos.0) && !boundary_check(pos.1){break}

        let temp_board: u64 = 1 << (pos.1 * 8) + pos.0;
        if pos == target {return true};
        if temp_board & board != 0 {
            obstructed = true;
        }
    }
    false
}

fn castle(bitboard: &[u64; 13], input: &Input, board: u64) -> bool {
    if (input.pos & bitboard[12] == 0) || (input.target & bitboard[12] == 0) {return false}
    let pos = find_coords(input.pos);
    let target = find_coords(input.target);
    if pos.1 != target.1 {return false};
    let displacement = target.0 - pos.0;
    let dir = displacement.signum();

    for i in 0..displacement{
        let x = input.pos << dir * i;
        if x & board != 0 && x & bitboard[12] == 0{return false}
    }
    true
}


fn move_piece(bitboard: &mut [u64; 13], input: Input, color: &Color){
    //Needs some thought to be actually good code
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
    let mut j = if *color == Color::White{0} else {6};
    j += match input.target_piece{
            Piece::Pawn => {0}
            Piece::Rook => {1}
            Piece::Knight => {2}
            Piece::Bishop => {3}
            Piece::Queen => {4}
            Piece::King => {5}
            _ => {12}
    };

    bitboard[i] ^= input.pos;
    
    //castling stuff
    if input.piece == Piece::King && input.target_piece == Piece::Rook && input.pos & bitboard[12] != 0 && input.target & bitboard[12] != 0{
        bitboard[i - 4] ^= input.target;
        let mut dir = -1;
        if (input.pos as i128 - input.target as i128) < 0 {dir = 1} 
        bitboard[i] |= shift(input.pos, dir * 2);
        bitboard[i - 4] |= shift(input.pos, dir);
    } else {
        bitboard[i] |= input.target;
        bitboard[j] ^= input.target;
    }

    //This is in case there is a special state that needs to be taken care of (aka of the 13th board)
    if bitboard[12] & input.pos != 0 && input.piece != Piece::Pawn{
        bitboard[12] ^= input.pos;
        bitboard[12] ^= input.target;
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
        print!("\n");
    }
}
