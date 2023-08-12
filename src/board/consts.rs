use crate::{
    consts::{Bitboard, NrOf, Piece, Square, MASK_8},
    utils::const_str_equal,
};

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

pub struct Squares;
impl Squares {
    pub const A1: Square = 0;
    pub const B1: Square = 1;
    pub const C1: Square = 2;
    pub const D1: Square = 3;
    pub const E1: Square = 4;
    pub const F1: Square = 5;
    pub const G1: Square = 6;
    pub const H1: Square = 7;

    pub const A8: Square = 56;
    pub const B8: Square = 57;
    pub const C8: Square = 58;
    pub const D8: Square = 59;
    pub const E8: Square = 60;
    pub const F8: Square = 61;
    pub const G8: Square = 62;
    pub const H8: Square = 63;
}

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

// Bitboards with a 1 in the nth square
const fn init_square_bbs() -> [Bitboard; NrOf::SQUARES] {
    let mut res = [0; NrOf::SQUARES];
    let mut i = 0;
    while i < NrOf::SQUARES {
        res[i] = 1u64 << i;
        i += 1;
    }
    res
}

// Bitboards with files filled with 1s
const fn init_file_bbs() -> [Bitboard; NrOf::FILES] {
    let mut res = [0; NrOf::FILES];
    let mut i = 0;
    while i < NrOf::FILES {
        let mut j = 0;
        while j < NrOf::SQUARES {
            res[i] |= 1 << (i + j);
            j += 8;
        }
        i += 1;
    }
    res
}

// Bitboards with ranks filled with 1s
const fn init_rank_bbs() -> [Bitboard; NrOf::RANKS] {
    let mut res = [0; NrOf::RANKS];
    let mut i = 0;
    while i < NrOf::RANKS {
        res[i] |= MASK_8 << (i * 8);
        i += 1;
    }
    res
}

pub const SQUARE_BBS: [Bitboard; NrOf::SQUARES] = init_square_bbs();
pub const FILE_BBS: [Bitboard; NrOf::FILES] = init_file_bbs();
pub const RANK_BBS: [Bitboard; NrOf::RANKS] = init_rank_bbs();

pub struct Files;
impl Files {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const G: usize = 6;
    pub const H: usize = 7;
}

pub struct Ranks;
impl Ranks {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}
