pub mod defs;
mod init_moves;
mod legalmoves;
mod list;
pub mod magics;

use self::list::MoveList;
use crate::{
    board::Board,
    defs::{Bitboard, Colors, NrOf},
};

pub const BISHOP_TABLE_SIZE: usize = 5248;
pub const ROOK_TABLE_SIZE: usize = 102400;

pub struct MoveGenerator {
    king: [Bitboard; NrOf::SQUARES],
    knight: [Bitboard; NrOf::SQUARES],
    pawn_capture: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
    bishop: [Bitboard; BISHOP_TABLE_SIZE],
    rook: [Bitboard; ROOK_TABLE_SIZE],
    pub rook_masks: [Bitboard; NrOf::SQUARES],
    pub bishop_masks: [Bitboard; NrOf::SQUARES],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king: [0; NrOf::SQUARES],
            knight: [0; NrOf::SQUARES],
            pawn_capture: [[0; NrOf::SQUARES]; Colors::BOTH],
            bishop: [0; BISHOP_TABLE_SIZE],
            rook: [0; ROOK_TABLE_SIZE],
            rook_masks: [0; NrOf::SQUARES],
            bishop_masks: [0; NrOf::SQUARES],
        };
        mg.init_king();
        mg.init_knight();
        mg.init_pawn_captures();
        mg.init_masks(); // masks for sliding pieces
        mg.init_sliding();
        mg
    }

    pub fn get_all_legal_moves(&self, board: &Board, only_captures: bool) -> MoveList {
        let mut list = MoveList::new();
        for piece in 0..NrOf::PIECE_TYPES {
            self.piece_legal_moves(&mut list, board, piece, only_captures)
        }
        list
    }
}
