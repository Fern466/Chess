fn main() {
    /*
    Order for pieces:
    Pawn, Rook, Knight, Bishop, Queen, King
    0-5 is white
    6-10 is black
    11 is e.p. and castling
     */
    let mut bitboard = bitboard();

}

fn bitboard() -> [u64; 13]{
    let mut bitboard = [0u64; 13];

    //White
    bitboard[0] = 0b11111111 << 8;
    bitboard[1] = 0b10000001;
    bitboard[2] = 0b01000010;
    bitboard[3] = 0b00100100;
    bitboard[4] = 0b00010000;
    bitboard[4] = 0b00001000;

    //Black
    bitboard[0] = 0b11111111 << 48;
    bitboard[1] = 0b10000001;
    bitboard[2] = 0b01000010;
    bitboard[3] = 0b00100100;
    bitboard[4] = 0b00001000;
    bitboard[4] = 0b00010000;

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
