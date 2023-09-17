use crate::{
    board::defs::{Files, Pieces, Ranks, FILE_BBS, RANK_BBS, SQUARE_BBS},
    defs::{Bitboard, Colors, NrOf, Piece, Square},
    utils::{add_square_i8, bit_ops},
};

use super::{
    defs::MoveDirection,
    magics::{BISHOP_MAGICS, ROOK_MAGICS},
    MoveGenerator,
};

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

    pub fn init_masks(&mut self) {
        for (piece, arr) in [
            (Pieces::ROOK, &mut self.rook_masks),
            (Pieces::BISHOP, &mut self.bishop_masks),
        ] {
            for sq in 0..NrOf::SQUARES {
                arr[sq] = Self::piece_rays_bb(piece, sq, 0);
            }
        }
    }

    pub fn init_sliding(&mut self) {
        let (piece, masks, table) = (Pieces::ROOK, &self.rook_masks, &mut self.rook);
        for sq in 0..NrOf::SQUARES {
            let bb = Self::simplify_blocker(masks[sq], sq);
            for blocker in Self::generate_blockers(bb) {
                let legal = Self::piece_rays_bb(piece, sq, blocker);
                let magic = ROOK_MAGICS[sq];
                let i = magic.get_index(blocker);
                if table[i] != 0 {
                    panic!("Magic indexing error.");
                }
                table[i] = legal;
            }
        }

        let (piece, masks, table) = (Pieces::BISHOP, &self.bishop_masks, &mut self.bishop);
        for sq in 0..NrOf::SQUARES {
            let bb = Self::simplify_blocker(masks[sq], sq);
            for blocker in Self::generate_blockers(bb) {
                let legal = Self::piece_rays_bb(piece, sq, blocker);
                let magic = BISHOP_MAGICS[sq];
                let i = magic.get_index(blocker);
                if table[i] != 0 {
                    panic!("Magic indexing error.");
                }
                table[i] = legal;
            }
        }
    }

    pub fn generate_blockers(bb: Bitboard) -> Vec<u64> {
        let bb_ones = bit_ops::one_indexes(bb);
        // the possible blocker bits are in the n bits that are 1 in the original Bitboard
        // so the maximum number of blockers is 2^n
        let n_blockers = 1 << bb_ones.len();

        let mut blocker_bbs = vec![0; n_blockers];
        for blocker_index in 0..n_blockers {
            for bit_index in 0..bb_ones.len() {
                let bit = ((blocker_index >> bit_index) & 1) as u64;
                blocker_bbs[blocker_index] |= bit << bb_ones[bit_index];
            }
        }
        blocker_bbs
    }

    // This function simplifies the blocker Bitboard by removing the edges when possible
    // since the last square of the blocker is indifferent for the move generation
    // see explanation for this and more in this video: https://www.youtube.com/watch?v=_vqlIPDR2TU&t=1725s
    pub fn simplify_blocker(bb: Bitboard, sq: Square) -> u64 {
        let mut res = bb;
        let sq_bb = SQUARE_BBS[sq];
        for zone in [
            FILE_BBS[Files::H],
            FILE_BBS[Files::A],
            RANK_BBS[Ranks::R1],
            RANK_BBS[Ranks::R8],
        ] {
            if !(sq_bb & zone > 0) {
                res &= !zone;
            }
        }
        res
    }

    pub fn piece_rays_bb(piece: Piece, sq: Square, blocker: Bitboard) -> Bitboard {
        let mut res = 0;
        for dir in MoveDirection::from_pos(sq, piece) {
            let mut ray_sq = sq;
            while let Some(i) = add_square_i8(ray_sq, dir.bb_val()) {
                res |= SQUARE_BBS[i];
                if Self::reached_edge(i, &dir) || blocker & SQUARE_BBS[i] > 0 {
                    break;
                }
                ray_sq = i;
            }
        }
        res
    }

    // This function returns true if the ray reached the side of the board in the given direction
    pub fn reached_edge(sq: Square, dir: &MoveDirection) -> bool {
        use MoveDirection::*;
        let bishop =
            SQUARE_BBS[sq] & FILE_BBS[Files::A] > 0 || SQUARE_BBS[sq] & FILE_BBS[Files::H] > 0;
        let rook = bishop && (dir == &E || dir == &W);
        match dir {
            N | E | S | W => rook,
            _ => bishop,
        }
    }
}
