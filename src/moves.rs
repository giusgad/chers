pub mod consts;
mod init_moves;
mod legalmoves;
mod list;

use crate::{
    board::{consts::Pieces, Board},
    consts::{Bitboard, Colors, NrOf},
};

use self::list::MoveList;

pub struct MoveGenerator {
    // TODO: remove pub
    pub king: [Bitboard; NrOf::SQUARES],
    pub knight: [Bitboard; NrOf::SQUARES],
    pub pawn_quiet: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
    pub pawn_capture: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king: [0; NrOf::SQUARES],
            knight: [0; NrOf::SQUARES],
            pawn_quiet: [[0; NrOf::SQUARES]; Colors::BOTH],
            pawn_capture: [[0; NrOf::SQUARES]; Colors::BOTH],
        };
        mg.init_king();
        mg.init_knight();
        mg.init_pawn_quiet();
        mg.init_pawn_captures();
        mg
    }

    pub fn get_all_legal_moves(&self, board: &Board) -> MoveList {
        let mut list = MoveList::new();
        self.piece_legal_moves(&mut list, board, Pieces::KING);
        self.piece_legal_moves(&mut list, board, Pieces::QUEEN);
        self.piece_legal_moves(&mut list, board, Pieces::BISHOP);
        self.piece_legal_moves(&mut list, board, Pieces::KNIGHT);
        self.piece_legal_moves(&mut list, board, Pieces::ROOK);
        list
    }
}
