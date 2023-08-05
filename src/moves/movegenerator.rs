use crate::consts::{Bitboard, Colors, NrOf};

pub struct MoveGenerator {
    // TODO: remove pub
    pub king: [Bitboard; NrOf::SQUARES],
    pub queen: [Bitboard; NrOf::SQUARES],
    pub rook: [Bitboard; NrOf::SQUARES],
    pub bishop: [Bitboard; NrOf::SQUARES],
    pub knight: [Bitboard; NrOf::SQUARES],
    pub pawn_quiet: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
    pub pawn_capture: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king: [0; NrOf::SQUARES],
            queen: [0; NrOf::SQUARES],
            rook: [0; NrOf::SQUARES],
            bishop: [0; NrOf::SQUARES],
            knight: [0; NrOf::SQUARES],
            pawn_quiet: [[0; NrOf::SQUARES]; Colors::BOTH],
            pawn_capture: [[0; NrOf::SQUARES]; Colors::BOTH],
        };
        mg.init_king();
        mg.init_knight();
        mg.init_bishop();
        mg.init_rook();
        mg.init_queen();
        mg.init_pawn_quiet();
        mg.init_pawn_captures();
        mg
    }
}
