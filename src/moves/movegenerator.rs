use crate::{
    board::consts::SQUARE_BBS,
    consts::{Bitboard, Colors, NrOf},
    utils::{add_square_i8, print_bb},
};

use super::consts::MoveDirection;

pub struct MoveGenerator {
    pub king: [Bitboard; NrOf::SQUARES],
    queen: [Bitboard; NrOf::SQUARES],
    bishop: [Bitboard; NrOf::SQUARES],
    knight: [Bitboard; NrOf::SQUARES],
    pawn: [[Bitboard; NrOf::SQUARES]; Colors::BOTH],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king: [0; NrOf::SQUARES],
            queen: [0; NrOf::SQUARES],
            bishop: [0; NrOf::SQUARES],
            knight: [0; NrOf::SQUARES],
            pawn: [[0; NrOf::SQUARES]; Colors::BOTH],
        };
        mg.init_king();
        mg
    }

    fn init_king(&mut self) {
        for sq in 0..NrOf::SQUARES {
            for direction in MoveDirection::from_pos(sq).iter() {
                if let Some(i) = add_square_i8(sq, direction.bb_val()) {
                    self.king[sq] |= SQUARE_BBS[i];
                }
            }
        }
    }
}
