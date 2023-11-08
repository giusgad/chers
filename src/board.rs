pub mod defs;
mod fen;
mod history;
mod makemove;
mod state;
mod zobrist;

use self::{
    defs::{PieceNames, Pieces, SQUARE_BBS, SQUARE_NAMES},
    history::History,
    state::State,
    zobrist::Zobrist,
};
use crate::{
    defs::{Bitboard, Color, Colors, NrOf, Piece, Square, PIECE_VALUES},
    eval::psqt::{FLIP, PSQTS_EG, PSQTS_MG},
    utils::bit_ops::find_ones_u8,
};

pub struct Board {
    pub piece_bbs: [[Bitboard; NrOf::PIECE_TYPES]; Colors::BOTH],
    pub color_bbs: [Bitboard; Colors::BOTH],
    pub state: State,
    pub history: History,
    zobrist: Zobrist,
    pub pieces: [[Piece; NrOf::SQUARES]; Colors::BOTH],
}

impl Board {
    pub fn new() -> Self {
        let mut zb = Zobrist::default();
        zb.init();
        Self {
            piece_bbs: [[0u64; NrOf::PIECE_TYPES]; Colors::BOTH],
            color_bbs: [0; Colors::BOTH],
            state: State::default(),
            history: History::default(),
            zobrist: zb,
            pieces: [[Pieces::NONE; NrOf::SQUARES]; Colors::BOTH],
        }
    }

    fn put_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] |= SQUARE_BBS[square];
        self.color_bbs[color] |= SQUARE_BBS[square];
        self.pieces[color][square] = piece;

        self.state.material[color] += PIECE_VALUES[piece];
        self.state.zobrist_hash ^= self.zobrist.piece_hash(color, piece, square);

        self.state.psqt_mg[color] += Self::get_psqt_val(piece, color, square, false);
        self.state.psqt_eg[color] += Self::get_psqt_val(piece, color, square, true);
    }

    fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] ^= SQUARE_BBS[square];
        self.color_bbs[color] ^= SQUARE_BBS[square];
        self.pieces[color][square] = Pieces::NONE;

        self.state.material[color] -= PIECE_VALUES[piece];
        self.state.zobrist_hash ^= self.zobrist.piece_hash(color, piece, square);

        self.state.psqt_mg[color] -= Self::get_psqt_val(piece, color, square, false);
        self.state.psqt_mg[color] -= Self::get_psqt_val(piece, color, square, true);
    }

    pub fn get_piece_bb(&self, piece: Piece, color: Color) -> Bitboard {
        self.piece_bbs[color][piece]
    }

    pub fn king_square(&self, color: Color) -> Square {
        self.piece_bbs[color][Pieces::KING].trailing_zeros() as Square
    }

    pub fn set_ep_square(&mut self, sq: Square) {
        self.state.ep_square = Some(sq);
        self.state.zobrist_hash ^= self.zobrist.en_passant_hash(sq);
    }

    pub fn clear_ep_square(&mut self) {
        if let Some(sq) = self.state.ep_square {
            self.state.zobrist_hash ^= self.zobrist.en_passant_hash(sq);
        }
        self.state.ep_square = None;
    }

    fn get_psqt_val(piece: Piece, color: Color, square: Square, is_endgame: bool) -> i16 {
        let square = if color == Colors::WHITE {
            FLIP[square]
        } else {
            square
        };
        if is_endgame {
            PSQTS_EG[piece][square]
        } else {
            PSQTS_MG[piece][square]
        }
    }

    pub fn zobrist_from_scratch(&self) -> u64 {
        let mut zob = 0;
        for color in 0..Colors::BOTH {
            for (sq, piece) in self.pieces[color].into_iter().enumerate() {
                if piece != Pieces::NONE {
                    zob ^= self.zobrist.piece_hash(color, piece, sq);
                }
            }
        }
        zob ^= self.zobrist.castling_hash(self.state.castling);
        if let Some(sq) = self.state.ep_square {
            zob ^= self.zobrist.en_passant_hash(sq);
        }
        if self.state.active_color == Colors::BLACK {
            zob ^= self.zobrist.color_hash();
        }
        zob
    }
}

impl Board {
    pub fn to_string(&self, unicode: bool) -> String {
        let mut board_chars = [' '; 64];
        let (black_chars, white_chars) = if unicode {
            (PieceNames::UNICODE_BLACK, PieceNames::UNICODE_WHITE)
        } else {
            (PieceNames::CHAR_LOWERCASE, PieceNames::CHAR_UPPERCASE)
        };
        for (piecetype, bb) in self.piece_bbs[Colors::WHITE].iter().enumerate() {
            for rank_nr in 0..8 {
                // shift the bitboard to the right to align the rank and mask it to preserve 8 bits
                let rank = (bb >> (8 * rank_nr)) & u64::from(u8::MAX);
                for file_nr in find_ones_u8(rank) {
                    let i = rank_nr * 8 + file_nr;
                    assert!(
                        board_chars[i] == ' ',
                        "two pieces on {} printing board",
                        SQUARE_NAMES[i]
                    );
                    board_chars[i] = white_chars[piecetype];
                }
            }
        }
        for (piecetype, bb) in self.piece_bbs[Colors::BLACK].iter().enumerate() {
            for rank_nr in 0..8 {
                // shift the bitboard to the right to align the rank and mask it to preserve 8 bits
                let rank = (bb >> (8 * rank_nr)) & u64::from(u8::MAX);
                for file_nr in find_ones_u8(rank) {
                    let i = rank_nr * 8 + file_nr;
                    assert!(
                        board_chars[i] == ' ',
                        "two pieces on {} printing board",
                        SQUARE_NAMES[i]
                    );
                    board_chars[i] = black_chars[piecetype];
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
        ranks.join("\n")
    }
}
