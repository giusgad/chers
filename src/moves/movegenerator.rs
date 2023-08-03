use crate::consts::{Bitboard, Colors, NrOf};

use super::consts::MoveDirection;

pub struct LegalMoves {
    king: [Bitboard; NrOf::SQUARES],
    queen: [Bitboard; NrOf::SQUARES],
    bishop: [Bitboard; NrOf::SQUARES],
    knight: [Bitboard; NrOf::SQUARES],
    pawn: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
}

impl LegalMoves {
    fn init_king(&mut self) {
        for sq in 0..NrOf::SQUARES {}
    }
}
