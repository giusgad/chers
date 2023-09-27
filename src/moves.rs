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
    bishop: Vec<Bitboard>,
    rook: Vec<Bitboard>,
    pub rook_masks: [Bitboard; NrOf::SQUARES],
    pub bishop_masks: [Bitboard; NrOf::SQUARES],
}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self {
            king: [0; NrOf::SQUARES],
            knight: [0; NrOf::SQUARES],
            pawn_capture: [[0; NrOf::SQUARES]; Colors::BOTH],
            bishop: vec![0; BISHOP_TABLE_SIZE],
            rook: vec![0; ROOK_TABLE_SIZE],
            rook_masks: [0; NrOf::SQUARES],
            bishop_masks: [0; NrOf::SQUARES],
        }
    }
}

impl MoveGenerator {
    pub fn init(&mut self) {
        self.init_king();
        self.init_knight();
        self.init_pawn_captures();
        self.init_masks(); // masks for sliding pieces
        self.init_sliding();
    }

    pub fn get_all_legal_moves(&self, board: &Board, only_captures: bool) -> MoveList {
        let mut list = MoveList::default();
        for piece in 0..NrOf::PIECE_TYPES {
            self.piece_legal_moves(&mut list, board, piece, only_captures)
        }
        list
    }
}
