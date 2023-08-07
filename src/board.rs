pub mod consts;
mod fen;
mod history;
mod makemove;
mod state;

use crate::{
    board::{
        consts::{PieceNames, Pieces, SQUARE_BBS},
        history::History,
        state::State,
    },
    consts::{Bitboard, Color, Colors, NrOf, Piece, Square, PIECE_VALUES},
    utils::bit_ops::find_ones,
};

pub struct Board {
    piece_bbs: [[Bitboard; NrOf::PIECE_TYPES]; Colors::BOTH],
    pub color_bbs: [Bitboard; Colors::BOTH],
    pub state: State,
    history: History,
    pub pieces: [[Piece; NrOf::SQUARES]; Colors::BOTH],
}

impl Board {
    pub fn new() -> Self {
        Self {
            piece_bbs: [[0u64; NrOf::PIECE_TYPES]; Colors::BOTH],
            color_bbs: [0; Colors::BOTH],
            state: State::new(),
            history: History::new(),
            pieces: [[Pieces::NONE; NrOf::SQUARES]; Colors::BOTH],
        }
    }

    fn put_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] |= SQUARE_BBS[square];
        self.color_bbs[color] |= SQUARE_BBS[square];
        self.pieces[color][square] = piece;
        self.state.material[color] += PIECE_VALUES[piece];
    }

    fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] ^= SQUARE_BBS[square];
        self.color_bbs[color] ^= SQUARE_BBS[square];
        self.pieces[color][square] = Pieces::NONE;
        self.state.material[color] -= PIECE_VALUES[piece];
    }

    // TODO:remove dbg function
    pub fn print_pawns(&self) {
        println!("{:064b}", self.piece_bbs[Colors::WHITE][Pieces::BISHOP]);
        println!("{:064b}", self.piece_bbs[Colors::BLACK][Pieces::BISHOP]);
    }

    pub fn get_pieces(&self, piece: Piece, color: Color) -> Bitboard {
        self.piece_bbs[color][piece]
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
