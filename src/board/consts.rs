use crate::{
    consts::{Bitboard, NrOf, Piece, Square},
    utils::const_str_equal,
};

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

pub struct PieceNames;
impl PieceNames {
    pub const FULL: [&'static str; NrOf::PIECE_TYPES + 1] =
        ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn", ""];
    pub const CHAR_UPPERCASE: [char; NrOf::PIECE_TYPES + 1] = ['K', 'Q', 'R', 'B', 'N', 'P', ' '];
    pub const CHAR_LOWERCASE: [char; NrOf::PIECE_TYPES + 1] = ['k', 'q', 'r', 'b', 'n', 'p', ' '];
}

pub struct Castling;
// 4 bits BK|BQ|WK|WQ
impl Castling {
    pub const WQ: u8 = 1;
    pub const WK: u8 = 2;
    pub const BQ: u8 = 4;
    pub const BK: u8 = 8;
    pub const ALL: u8 = 15;
}

#[rustfmt::skip]
pub const SQUARE_NAMES: [&str; NrOf::SQUARES] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];

pub const fn square_by_name(name: &str) -> Result<Square, ()> {
    let mut i = 0;
    while i < NrOf::SQUARES {
        if const_str_equal(name, SQUARE_NAMES[i]) {
            return Ok(i);
        }
        i += 1;
    }
    Err(())
}

const fn init_square_bbs() -> [Bitboard; NrOf::SQUARES] {
    let mut res = [0; NrOf::SQUARES];
    let mut i = 0;
    while i < NrOf::SQUARES {
        res[i] = 1u64 << i;
        i += 1;
    }
    res
}

pub const SQUARE_BBS: [Bitboard; NrOf::SQUARES] = init_square_bbs();
