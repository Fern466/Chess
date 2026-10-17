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
use lib::assemble_bitmask;
use lib::fill_1s;
use lib::shift_u8;
fn main() {
    /*
    Order for pieces:
    Pawn, Rook, Knight, Bishop, Queen, King
    0-5 is black
    6-10 is white
    11 is e.p. and castling
     */

    //Use to test the validity of a bitboard
    let bruh = assemble_bitmask((4, 4), &Color::White);
    let bruh = bruh[0];
    for y in 0..8{
        for x in 0..8 {
            let mut letter = ".".black();
            if (1 << (y * 8) + x) & bruh != 0{
                letter = "!".white();
            }
            print!("{letter}")
        }
        print!("\n");
    }

    let mut bitboard = bitboard();
    loop{
        display_board(&bitboard);
        player(&mut bitboard, &Color::White);
    }
}

fn player(bitboard: &mut [u64; 13], color: &Color) {
    loop {
        let input = get_input(bitboard);
        let mut trial = *bitboard;
        if validate_input(&mut trial, &input, color) {
            *bitboard = trial;
            move_piece(bitboard, input, color);
            return;
        }
    }
}

const KING: [(i32, i32); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
fn validate_input(bitboard: &mut [u64; 13], input: &Input, color: &Color) -> bool{
    //first will be friendlies, last enemies
    let mut boards = (0, 0);
    let black = bitboard[0] | bitboard[1] | bitboard[2] | bitboard[3] | bitboard[4] | bitboard[5];
    let white = bitboard[6] | bitboard[7] | bitboard[8] | bitboard[9] | bitboard[10] | bitboard[11];
    boards.0 = if *color == Color::White{white} else {black};
    boards.1 = if *color == Color::White{black} else {white};

    //This if statement is just a "gatekeeper" since otherwise we'd need to check this later
    if boards.0 & input.target == 0{
        match input.piece {
            Piece::Pawn => {return pawn(bitboard, input, color, &boards.1, &boards.0)}
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
            return castle(bitboard, input, boards.0 | boards.1, color);
        }
    }
    false
}

fn pawn(bitboard: &mut [u64; 13], input: &Input, color: &Color, enemy_board: &u64, friendly_board: &u64) -> bool{
    let offset = if *color == Color::Black {1} else {-1};
    let x = find_coords(input.pos);
    let not_overhead = |x: i32, i: i32| if (x + i) >= 0 && (x + i) <= 7 {true} else {false};
    //this code is jank. I'm just lazy and don't want to have to find the exact enemy pawns so I just get a list of all the pawns and compare it to enemy board.
    let enemy_pawn = if *color == Color::White {bitboard[0]} else {bitboard[6]};
    let mut left_ep = false;
    if not_overhead(x.0, -1){
        left_ep = shift(input.pos, -1) & (enemy_board & enemy_pawn) != 0 && shift(input.pos,-1) & bitboard[12] != 0;
        left_ep = left_ep && shift(input.pos, (offset * 8) - 1) == input.target;
    }
    let mut right_ep = false;
    if not_overhead(x.0, 1){
        right_ep = shift(input.pos, 1) & (enemy_board & enemy_pawn) != 0 && shift(input.pos, 1) & bitboard[12] != 0;
        right_ep = right_ep && shift(input.pos, (offset * 8) +  1) == input.target;
    }
    if (left_ep || right_ep ) && input.target_piece == Piece::None{return true}
    
    clean_bitboard(bitboard);

    let temp = find_coords(input.pos);
    let correct_pos: bool = (temp.1 == 1 && *color == Color::Black) || (temp.1 == 6 && *color == Color::White);

    if shift(input.pos, offset * 16) == input.target && correct_pos && (enemy_board & input.target) == 0 && (shift(input.pos, offset * 8) & (enemy_board | friendly_board)) == 0 {
        bitboard[12] |= input.target;
        return true;
    }    
    if shift(input.pos, offset * 8) == input.target && (enemy_board & input.target) == 0 {return true}
    if shift(input.pos, (offset * 8) + 1) == input.target && (enemy_board & input.target) != 0 && not_overhead(x.0, 1) {return true}
    if shift(input.pos, (offset * 8) - 1) == input.target && (enemy_board & input.target) != 0 && not_overhead(x.0, -1) {return true}
    false
}

const KNIGHT_TRANSFORM: [(i32, i32); 8] = [(2, 1), (1, 2), (2, -1), (1 , -2), (-2, -1), (-1, -2), (-2, 1), (-1, 2)];
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
        if !boundary_check(pos.0) || !boundary_check(pos.1){break}

        let temp_board: u64 = 1 << (pos.1 * 8) + pos.0;
        if pos == target {return true};
        if temp_board & board != 0 {
            obstructed = true;
        }
    }
    false
} 

fn castle(bitboard: &[u64; 13], input: &Input, board: u64, color: &Color) -> bool {
    if (input.pos & bitboard[12] == 0) || (input.target & bitboard[12] == 0) {return false}
    let pos = find_coords(input.pos);
    let target = find_coords(input.target);
    if pos.1 != target.1 {return false};
    let displacement = target.0 - pos.0;
    let dir = displacement.signum();


    // every square strictly between king and rook must be empty
    for i in 1..displacement.abs() {
        if shift(input.pos, dir * i) & board != 0 { return false }
    }

    // the king may not start in, pass through, or land on an attacked square
    let king_idx = if *color == Color::White { 10 } else { 4 };
    for step in 0..=2 {
        let mut trial = *bitboard;
        trial[king_idx] = shift(input.pos, dir * step);
        if check(&trial, color) { return false }
    }
    true
}


fn move_piece(bitboard: &mut [u64; 13], input: Input, color: &Color){
    //Absolutely horrendous code quality
    let mut board: u64 = 0;
    for i in 0..13{board |= bitboard[i]}
    let mut i = if *color == Color::White{6} else {0};
    i += match input.piece{
            Piece::Pawn => {0}
            Piece::Rook => {1}
            Piece::Knight => {2}
            Piece::Bishop => {3}
            Piece::Queen => {5}
            Piece::King => {4}
            _ => {12}
    };
    let mut j = if *color == Color::White{0} else {6};
    j += match input.target_piece{
            Piece::Pawn => {0}
            Piece::Rook => {1}
            Piece::Knight => {2}
            Piece::Bishop => {3}
            Piece::Queen => {5}
            Piece::King => {4}
            _ => {255}
    };

    bitboard[i] ^= input.pos;
    
    //castling stuff
    if input.piece == Piece::King && input.target_piece == Piece::Rook && input.pos & bitboard[12] != 0 && input.target & bitboard[12] != 0{
        bitboard[i - 3] ^= input.target;
        let dir = (input.target as i128 - input.pos as i128).signum() as i32;
        bitboard[i] = shift(input.pos, dir * 2);
        bitboard[i - 3] ^= shift(input.pos, dir);
    } else {
        bitboard[i] |= input.target;
        if j < 255 {bitboard[j] ^= input.target};
    }

    //e.p.
    let temp0 = find_coords(input.pos);
    let temp1 = find_coords(input.target);
    let on_same_x = if (temp0.0 - temp1.0) == 0 {true} else {false};
    if input.piece == Piece::Pawn && input.target & board == 0 && !on_same_x {
        let x = if *color == Color::White{1} else {-1};
        let z = if *color == Color::White{0} else {6};
        bitboard[z] ^= shift(input.target, x * 8);
    }

    //This is in case there is a special state that needs to be taken care of (aka of the 13th board)
    if bitboard[12] & input.pos != 0 && input.piece != Piece::Pawn{
        bitboard[12] ^= input.pos;
        if bitboard[12] & input.target != 0{
            bitboard[12] ^= input.target;
        }
    }
}

//Cleans the pawn part of the 13th board
fn clean_bitboard(bitboard: &mut [u64; 13]){
    for i in 3..6{
        let temp = 0b11111111 << (8 * i);
        bitboard[12] &= !temp;
    }
}

//If true, should be processed by the checkmate function
fn check(bitboard: &[u64; 13], color: &Color) -> bool{
    //creates seperate boards for easier and clearer checking
    let enemy = if *color == Color::White{&bitboard[0..6]} else {&bitboard[6..12]};
    let mut all_pieces: u64 = 0;
    for i in 0..12{all_pieces |= bitboard[i]}
    let king = if *color == Color::White{bitboard[10]} else {bitboard[4]};

    //Pawn, Knight & King
    let bitmasks = assemble_bitmask(find_coords(king), color);
    if bitmasks[0] & enemy[0] != 0 ||
    bitmasks[1] & enemy[2] != 0 ||
    bitmasks[2] & enemy[4] != 0 
    {return true}
    
    //Bishop, Rook & Queen
    let vertical_and_horizontal = bitmasks[3] | bitmasks[4] | bitmasks[5] | bitmasks[6];
    let diagonal = bitmasks[7] | bitmasks[8] | bitmasks[9] | bitmasks[10];

    let diagonal_pieces = enemy[3] | enemy[5];
    let vertical_and_horizontal_pieces = enemy[1] | enemy[5];

    if slider_attacks(king, vertical_and_horizontal_pieces, &bitmasks[3..7], all_pieces) {return true};
    if slider_attacks(king, diagonal_pieces, &bitmasks[7..11], all_pieces) {return true};
    false
}

fn slider_attacks(king: u64, pieces: u64, rays: &[u64], all_pieces: u64) -> bool {
    let k = king.trailing_zeros() as i32;
    for &ray in rays {
        let hits = ray & pieces;
        let above = hits & !(king | (king - 1));
        let below = hits & (king - 1);
        if above != 0 {
            let a = above.trailing_zeros() as i32; // nearest attacker above
            if fill_1s(k + 1, a) & ray & all_pieces == 0 { return true; }
        }
        if below != 0 {
            let b = 63 - below.leading_zeros() as i32; // nearest attacker below
            if fill_1s(b + 1, k) & ray & all_pieces == 0 { return true; }
        }
    }
    false
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
    let pieces: [ColoredString; 12] = ["P".black(), "R".black(), "K".black(), "B".black(), "$".black(), "Q".black(), "P".white(), "R".white(), "K".white(), "B".white(), "$".white(), "Q".white()];

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

#[cfg(test)]
mod tests {
    use super::*;

    //lib.rs code
    #[test]
    fn test_assemble_bitmasks() {
        let bitmasks = assemble_bitmask((4, 4), &Color::White);

        assert_eq!(bitmasks[0], 0b101 << 27);

        let knight: u64 = (0b101 << 19) | (0b10001 << 26) | (0b10001 << 42) | (0b101 << 51);
        assert_eq!(bitmasks[1], knight);

        let king: u64 = (0b111 << 27) | (0b101 << 35) | (0b111 << 43);
        assert_eq!(bitmasks[2], king);

        let rook: u64 = (1 << 4) | (1 << 12) | (1 << 20) | (1 << 28) | (0b11111111 << 32) | (1 << 44) | (1 << 52) | (1 << 60);
        assert_eq!(bitmasks[3] | bitmasks[4] | bitmasks[5] | bitmasks[6], rook);

        let bishop: u64 = 1 | (0b010000010 << 8) | (0b001000100 << 16) | (0b000101000 << 24) | (0b000101000 << 40) | (0b01000100 << 48) | (0b10000010<< 56);
        assert_eq!(bitmasks[7] | bitmasks[8] | bitmasks[9] | bitmasks[10], bishop);

        let queen: u64 = bishop | rook;
        assert_eq!(bitmasks[3] | bitmasks[4] | bitmasks[5] | bitmasks[6] | bitmasks[7] | bitmasks[8] | bitmasks[9] | bitmasks[10], queen);
    }

    //some of these are probably unnecessary to test, but I'll do it anyways
    #[test]
    fn test_shift_u8(){
        assert_eq!(shift_u8(8, 20), 0);
        assert_eq!(shift(2, 2), 8);
        assert_eq!(shift(8, -2), 2);
    }

    #[test]
    fn test_shift(){
        assert_eq!(shift(2, 78), 0);
        assert_eq!(shift(2, 32), 8589934592);
        assert_eq!(shift(8589934592, -32), 2);
    }

    #[test]
    fn test_input(){
        let bitboard = bitboard();
        let input = input(vec![0, 0, 5, 7], &bitboard).unwrap();
        assert!(input.piece == Piece::Rook, "Incorrect piece at location");
        assert!(input.target_piece == Piece::Bishop, "Incorrect target piece at location");
        assert!(input.pos == 1, "Incorrect position recorded");
        assert!(input.target == 1 << 61, "Incorrect target positon recorded");
    }

    #[test]
    fn test_find_coords(){
        assert_eq!(find_coords(1 << (6 * 8) + 2), (2, 6));
        assert_eq!(find_coords(1 << (1 * 8) + 7), (7, 1));
    }

    #[test]
    fn test_fill_1s(){
        let mut x = 0;
        for i in 0..3{
            x += 0b11111111 << (i * 8);
        }
        assert_eq!(fill_1s(0, 24), x);

        let mut x = 0;
        for i in 4..7{
            x += 0b11111111 << (i * 8);
        }
        assert_eq!(fill_1s(32, 56), x);
    }

    //main.rs
    //This test should have a lot of tests
    #[test]
    fn test_validate_input(){
        let mut bitboard = bitboard();
        
        {//null input
            let input = input(vec![0, 4, 7, 7], &bitboard).unwrap();
            assert!(!validate_input(&mut bitboard, &input, &Color::White), "Null input reported as true");
        }{//pawn
            let input = input(vec![7, 6, 7, 4], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal pawn move reported as false");
        }{//knight
            let input = input(vec![1, 7, 0, 5], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal knight move reported as false");
        }{//queen unsuccessful
            let input = input(vec![3, 7, 3, 1], &bitboard).unwrap();
            assert!(!validate_input(&mut bitboard, &input, &Color::White), "Illegal queen move reported as true"); 
        }{//king unsucessful
            let input = input(vec![4, 7, 4, 6], &bitboard).unwrap();
            assert!(!validate_input(&mut bitboard, &input, &Color::White), "Illegal king move reported as true"); 
        }

        //modified board for king, queen, rook & bishop
        bitboard[6] = 0;
        {//king
            let input = input(vec![4, 7, 3, 6], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal king move reported as false"); 
        }{//queen
            let input = input(vec![3, 7, 0, 4], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal queen move reported as false"); 
        }{//rook
            let input = input(vec![0, 7, 0, 4], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal rook move reported as false"); 
        }{//bishop
            let input = input(vec![2, 7, 0, 5], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal bishop move reported as false"); 
        }{//pawn attacking
            bitboard[6] += 1 << 16;
            let input = input(vec![0, 2, 1, 1], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal pawn attack reported as false"); 
        }{//e.p. pawn
            bitboard[6] += 1 << 36;
            bitboard[0] += 1 << 37;
            bitboard[12] += 1 << 37;
            let input = input(vec![4, 4, 5, 3], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal en passant reported as false"); 
        }{//illegal castling
            let input = input(vec![0, 7, 4, 7], &bitboard).unwrap();
            assert!(!validate_input(&mut bitboard, &input, &Color::White), "Illegal castling move reported as true"); 
        }{//castling
            bitboard[8] = 0;
            bitboard[9] = 0;
            bitboard[12] = (1 << 60) + (1 << 63);
            let input = input(vec![4, 7, 7, 7], &bitboard).unwrap();
            assert!(validate_input(&mut bitboard, &input, &Color::White), "Legal castling move reported as false"); 
        }
    }

    #[test]
    fn test_move_piece(){
        let mut bitboard = bitboard();
        //pawn
        let player_input = input(vec![7, 6, 7, 4], &bitboard).unwrap();
        move_piece(&mut bitboard, player_input, &Color::White);
        assert!(bitboard[6] & (1 << 39) != 0, "Pawn move reported as incorrect");
        //knight
        let player_input = input(vec![1, 7, 0, 5], &bitboard).unwrap();
        move_piece(&mut bitboard, player_input, &Color::White);
        assert!(bitboard[8] & (1 << 40) != 0, "Knight move reported as incorrect");
        //e.p.
        bitboard[6] += 1 << 36;
        bitboard[0] += 1 << 37;
        bitboard[12] += 1 << 37;
        let player_input = input(vec![4, 4, 5, 3], &bitboard).unwrap();
        move_piece(&mut bitboard, player_input, &Color::White);
        assert!(bitboard[6] & (1 << 29) != 0, "Legal en passant reported as false");        
        //castling
        bitboard[8] = 0;
        bitboard[9] = 0;
        bitboard[12] = (1 << 60) + (1 << 63);
        let player_input = input(vec![4, 7, 7, 7], &bitboard).unwrap();
        move_piece(&mut bitboard, player_input, &Color::White);
        assert!(bitboard[10] & (1 << 62) != 0 && bitboard[7] & (1 << 61) != 0, "Legal castling move reported as false"); 
    }

    #[test]
    fn test_check(){
        let mut board: [u64; 13] = [0u64; 13];
        //pawn
        board[10] += 1 << 36;
        board[0] += 1 << 27;
        assert!(check(&board, &Color::White), "Correct pawn move incorrect");
        //knight
        board[0] = 0;
        board[2] += 1 << 21;
        assert!(check(&board, &Color::White), "Correct knight move incorrect");
        //king
        board[2] = 0;
        board[4] += 1 << 44; 
        assert!(check(&board, &Color::White), "Correct king move incorrect");
        //king incorrect
        board[4] = 0;
        board[4] += 1 << 50; 
        assert!(!check(&board, &Color::White), "Incorrect king move correct");
        //needs very rigorous checking since the code for this is particularly tricky and stupid section of code
        //queen incorrect (obstruction) diagonal
        board[4] = 0;
        board[2] += 1 << 45;
        board[5] += 1 << 54;
        assert!(!check(&board, &Color::White), "Incorrect queen move correct");
        //queen incorrect (obstruction) vertical
        board[5] = 0;
        board[2] = 0;
        board[2] += 1 << 44;
        board[5] += 1 << 52;
        assert!(!check(&board, &Color::White), "Incorrect queen move correct");
        //queen correct diagonal
        board[5] = 0;
        board[2] = 0;
        board[5] += 1 << 18;
        assert!(check(&board, &Color::White), "Correct queen move incorrect (diagonal)");
        //queen correct horizontal
        board[5] = 0;
        board[5] += 1 << 32;
        assert!(check(&board, &Color::White), "Correct queen move incorrect (horizontal)");
    }
}