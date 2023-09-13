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
    eval::psqt::{FLIP, KING_ENDGAME, PSQTS},
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
        Self {
            piece_bbs: [[0u64; NrOf::PIECE_TYPES]; Colors::BOTH],
            color_bbs: [0; Colors::BOTH],
            state: State::new(),
            history: History::new(),
            zobrist: Zobrist::new(),
            pieces: [[Pieces::NONE; NrOf::SQUARES]; Colors::BOTH],
        }
    }

    fn put_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] |= SQUARE_BBS[square];
        self.color_bbs[color] |= SQUARE_BBS[square];
        self.pieces[color][square] = piece;

        self.state.material[color] += PIECE_VALUES[piece];
        self.state.zobrist_hash ^= self.zobrist.piece_hash(color, piece, square);

        self.state.psqt[color] += self.get_psqt_val(piece, color, square);
    }

    fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bbs[color][piece] ^= SQUARE_BBS[square];
        self.color_bbs[color] ^= SQUARE_BBS[square];
        self.pieces[color][square] = Pieces::NONE;

        self.state.material[color] -= PIECE_VALUES[piece];
        self.state.zobrist_hash ^= self.zobrist.piece_hash(color, piece, square);

        self.state.psqt[color] -= self.get_psqt_val(piece, color, square);
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

    fn get_psqt_val(&self, piece: Piece, color: Color, square: Square) -> i16 {
        let square = if color == Colors::WHITE {
            FLIP[square]
        } else {
            square
        };
        if self.is_endgame() && piece == Pieces::KING {
            KING_ENDGAME[square]
        } else {
            PSQTS[piece][square]
        }
    }

    pub fn is_endgame(&self) -> bool {
        // endgame starts when both sides have no queens or the sides that have a queen have at
        // most one more minor piece
        // also there shouldn't be more than 6 minor+major pieces on the board
        let mut count = 0; //
        for color in 0..Colors::BOTH {
            for piece in 0..NrOf::PIECE_TYPES {
                if piece == Pieces::PAWN || piece == Pieces::KING {
                    continue;
                }
                count += self.piece_bbs[color][piece].count_ones();
            }
        }
        if count > 6 {
            return false;
        }

        let black_queen = self.piece_bbs[Colors::BLACK][Pieces::QUEEN] > 0;
        let white_queen = self.piece_bbs[Colors::WHITE][Pieces::QUEEN] > 0;

        let black_rook = self.piece_bbs[Colors::BLACK][Pieces::ROOK] > 0;
        let white_rook = self.piece_bbs[Colors::WHITE][Pieces::ROOK] > 0;

        let minors = [Pieces::KNIGHT, Pieces::BISHOP];

        let mut b_count = 0;
        let mut w_count = 0;
        for p in minors {
            b_count += self.piece_bbs[Colors::BLACK][p].count_ones();
            w_count += self.piece_bbs[Colors::WHITE][p].count_ones();
        }

        (!black_queen || (!black_rook && b_count <= 1))
            && (!white_queen || (!white_rook && w_count <= 1))
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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_chars = [' '; 64];
        for (piecetype, bb) in self.piece_bbs[Colors::WHITE].iter().enumerate() {
            for rank_nr in 0..8 {
                // shift the bitboard to the right to align the rank and mask it to preserve 8 bits
                let rank = (bb >> (8 * rank_nr)) & (u8::MAX as u64);
                for file_nr in find_ones_u8(rank) {
                    let i = rank_nr * 8 + file_nr;
                    if board_chars[i] != ' ' {
                        panic!("two pieces on {} printing board", SQUARE_NAMES[i])
                    }
                    board_chars[i] = PieceNames::CHAR_UPPERCASE[piecetype];
                }
            }
        }
        for (piecetype, bb) in self.piece_bbs[Colors::BLACK].iter().enumerate() {
            for rank_nr in 0..8 {
                // shift the bitboard to the right to align the rank and mask it to preserve 8 bits
                let rank = (bb >> (8 * rank_nr)) & (u8::MAX as u64);
                for file_nr in find_ones_u8(rank) {
                    let i = rank_nr * 8 + file_nr;
                    if board_chars[i] != ' ' {
                        panic!("two pieces on {} printing board", SQUARE_NAMES[i])
                    }
                    board_chars[i] = PieceNames::CHAR_LOWERCASE[piecetype];
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
