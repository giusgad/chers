use crate::consts::{Bitboard, NrOf, Piece};

pub const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

const fn init_squares_bb() -> [Bitboard; NrOf::SQUARES] {
    let mut res = [0; NrOf::SQUARES];
    let mut i = 0;
    while i < NrOf::SQUARES {
        res[i] = 1u64 << i;
        i += 1;
    }
    res
}

pub const SQUARES_BB: [Bitboard; NrOf::SQUARES] = init_squares_bb();
