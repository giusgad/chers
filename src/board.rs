mod consts;
mod fen;

use crate::{
    board::consts::PieceNames,
    consts::{Bitboard, Color, Colors, NrOf, Piece, Square},
    utils::find_ones,
};
use consts::SQUARES_BB;

pub struct Board {
    pub piece_bbs: [[Bitboard; NrOf::PIECE_TYPES]; Colors::BOTH],

    active_color: Color,
}

impl Board {
    pub fn new() -> Self {
        Self {
            piece_bbs: [[0u64; NrOf::PIECE_TYPES]; Colors::BOTH],
            active_color: Colors::WHITE,
        }
    }

    fn put_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] |= SQUARES_BB[square]
    }

    fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] ^= SQUARES_BB[square]
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_chars = [' '; 64];
        for (piecetype, bb) in self.piece_bbs[Colors::WHITE].iter().enumerate() {
            for rank_nr in 0..8 {
                // shift the bitboard to the right to align the rank and mask it to preserve 8 bits
                let rank = (bb >> (8 * rank_nr)) & (u8::MAX as u64);
                for file_nr in find_ones(rank) {
                    board_chars[rank_nr * 8 + file_nr] = PieceNames::CHAR_UPPERCASE[piecetype];
                }
            }
        }
        for (piecetype, bb) in self.piece_bbs[Colors::BLACK].iter().enumerate() {
            for rank_nr in 0..8 {
                // shift the bitboard to the right to align the rank and mask it to preserve 8 bits
                let rank = (bb >> (8 * rank_nr)) & (u8::MAX as u64);
                for file_nr in find_ones(rank) {
                    board_chars[rank_nr * 8 + file_nr] = PieceNames::CHAR_LOWERCASE[piecetype];
                }
            }
        }
        let mut ranks: [String; 8] = Default::default();
        let mut rank_nr = 8;
        for (i, c) in board_chars.iter().enumerate() {
            if i % 8 == 0 {
                rank_nr -= 1;
                ranks[rank_nr].push('|');
            }
            ranks[rank_nr].push(*c);
            ranks[rank_nr].push('|');
        }
        write!(f, "{}", ranks.join("\n"))
    }
}
