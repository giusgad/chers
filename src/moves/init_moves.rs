use crate::{
    board::defs::{Files, Pieces, FILE_BBS, SQUARE_BBS},
    defs::{Colors, NrOf, Square},
    utils::add_square_i8,
};

use super::{defs::MoveDirection, MoveGenerator};

// The init functions create bitboards with all the possible moves for every
// piece starting from every square. These can then be used to find legal moves
impl MoveGenerator {
    // for every square generate moves in every direction for one square
    pub fn init_king(&mut self) {
        for sq in 0..NrOf::SQUARES {
            // the from_pos function removes directions that cause horizontal overflow
            for direction in MoveDirection::from_pos(sq, Pieces::KING).iter() {
                if let Some(i) = add_square_i8(sq, direction.bb_val()) {
                    self.king[sq] |= SQUARE_BBS[i];
                }
            }
        }
    }

    /*
    knight bitboard moves pattern
      +15  +17
    +6        +10
    -10       -6
      -17  -15
    */
    pub fn knight_bb_vals(square: Square) -> Vec<i8> {
        let mut res: Vec<i8> = vec![6, -10, 15, -17, 17, -15, 10, -6];
        let bb_square = SQUARE_BBS[square];
        if bb_square & FILE_BBS[Files::G] > 0 {
            res.drain(6..8);
        } else if bb_square & FILE_BBS[Files::H] > 0 {
            res.drain(4..8);
        } else if bb_square & FILE_BBS[Files::B] > 0 {
            res.drain(0..2);
        } else if bb_square & FILE_BBS[Files::A] > 0 {
            res.drain(0..4);
        }
        res
    }

    // works like the others but instead of using directions uses the pattern given by knight_bb_vals
    pub fn init_knight(&mut self) {
        for sq in 0..NrOf::SQUARES {
            for direction in Self::knight_bb_vals(sq).iter() {
                if let Some(i) = add_square_i8(sq, *direction) {
                    self.knight[sq] |= SQUARE_BBS[i];
                }
            }
        }
    }

    pub fn init_pawn_captures(&mut self) {
        for sq in 0..NrOf::SQUARES {
            let bb_sq = SQUARE_BBS[sq];
            if !(bb_sq & FILE_BBS[Files::H] > 0) {
                if let Some(i) = add_square_i8(sq, MoveDirection::NE.bb_val()) {
                    self.pawn_capture[Colors::WHITE][sq] |= 1 << i;
                }
                if let Some(i) = add_square_i8(sq, MoveDirection::SE.bb_val()) {
                    self.pawn_capture[Colors::BLACK][sq] |= 1 << i;
                }
            }
            if !(bb_sq & FILE_BBS[Files::A] > 0) {
                if let Some(i) = add_square_i8(sq, MoveDirection::NW.bb_val()) {
                    self.pawn_capture[Colors::WHITE][sq] |= 1 << i;
                }
                if let Some(i) = add_square_i8(sq, MoveDirection::SW.bb_val()) {
                    self.pawn_capture[Colors::BLACK][sq] |= 1 << i;
                }
            }
        }
    }
}
