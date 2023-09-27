use super::defs::{Castling, FILE_BBS, SQUARE_BBS};
use crate::defs::{Color, Colors, NrOf, Piece, Square, ZobristHash};
use rand::{rngs::StdRng, Rng, SeedableRng};

const SEED: [u8; 32] = [213; 32];

// the random numbers to generate the keys are made of:
// - one number for each piece at each square
// - one number to indicate the side to move is black
// - four numbers for castling rights
// - eight numbers for the file of a valid en passant square
pub struct Zobrist {
    pieces: [[[u64; NrOf::SQUARES]; NrOf::PIECE_TYPES]; Colors::BOTH],
    color: u64,
    castling: [u64; 4],
    en_passant: [u64; 8],
}

impl Default for Zobrist {
    fn default() -> Self {
        Self {
            pieces: [[[0; NrOf::SQUARES]; NrOf::PIECE_TYPES]; Colors::BOTH],
            color: 0,
            castling: [0; 4],
            en_passant: [0; 8],
        }
    }
}

impl Zobrist {
    pub fn init(&mut self) {
        let mut rng = StdRng::from_seed(SEED);
        for col in 0..Colors::BOTH {
            for piece in 0..NrOf::PIECE_TYPES {
                for sq in 0..NrOf::SQUARES {
                    self.pieces[col][piece][sq] = rng.gen_range(0..u64::MAX);
                }
            }
        }
        self.color = rng.gen_range(0..u64::MAX);
        for i in 0..4 {
            self.castling[i] = rng.gen_range(0..u64::MAX);
        }
        for i in 0..8 {
            self.en_passant[i] = rng.gen_range(0..u64::MAX);
        }
    }

    pub fn piece_hash(&self, color: Color, piece: Piece, sq: Square) -> ZobristHash {
        self.pieces[color][piece][sq]
    }

    pub fn castling_hash(&self, perms: u8) -> ZobristHash {
        let mut hash = 0;
        if perms & Castling::WK > 0 {
            hash ^= self.castling[0];
        }
        if perms & Castling::WQ > 0 {
            hash ^= self.castling[1];
        }
        if perms & Castling::BK > 0 {
            hash ^= self.castling[2];
        }
        if perms & Castling::BQ > 0 {
            hash ^= self.castling[3];
        }
        hash
    }

    pub fn en_passant_hash(&self, sq: Square) -> ZobristHash {
        let sq = SQUARE_BBS[sq];
        for (i, file) in FILE_BBS.iter().enumerate() {
            if file & sq > 0 {
                return self.en_passant[i];
            }
        }
        0
    }

    pub fn color_hash(&self) -> ZobristHash {
        self.color
    }
}
