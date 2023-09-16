pub mod defs;
mod init_moves;
mod legalmoves;
mod list;

use std::collections::HashMap;

use crate::{
    board::Board,
    defs::{Bitboard, Colors, NrOf},
};

use self::list::MoveList;

pub struct MoveGenerator {
    king: [Bitboard; NrOf::SQUARES],
    knight: [Bitboard; NrOf::SQUARES],
    pawn_capture: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
    bishop_dict: HashMap<(usize, Bitboard), Bitboard>,
    rook_dict: HashMap<(usize, Bitboard), Bitboard>,
    rook_masks: [Bitboard; NrOf::SQUARES],
    bishop_masks: [Bitboard; NrOf::SQUARES],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king: [0; NrOf::SQUARES],
            knight: [0; NrOf::SQUARES],
            pawn_capture: [[0; NrOf::SQUARES]; Colors::BOTH],
            bishop_dict: HashMap::new(),
            rook_dict: HashMap::new(),
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
