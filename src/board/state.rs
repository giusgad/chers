use crate::{
    defs::{Color, Colors, ZobristHash},
    moves::defs::Move,
};

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub active_color: Color,
    pub castling: u8,
    pub ep_square: Option<usize>,
    pub halfmove_count: u8,
    pub fullmove_count: u16,
    pub zobrist_hash: ZobristHash,
    pub material: [u16; Colors::BOTH],
    pub psqt: [i16; Colors::BOTH],
    pub next_move: Move,
}

impl State {
    pub fn new() -> Self {
        Self {
            castling: 0,
            active_color: Colors::WHITE,
            ep_square: None,
            halfmove_count: 0,
            fullmove_count: 0,
            zobrist_hash: 0,
            material: [0; Colors::BOTH],
            psqt: [0; Colors::BOTH],
            next_move: Move { data: 0 },
        }
    }
}

impl Into<String> for State {
    fn into(self) -> String {
        format!("active color: {}\ncastling: {}\nep square: {:?}\nzobrist_hash: {}\nhalfmove clock: {}\nfullmove count: {}\nmaterial: {:?}, psqt: {:?}",
            self.active_color,
            self.castling,
            self.ep_square,
            self.zobrist_hash,
            self.halfmove_count,
            self.fullmove_count,
            self.material,
            self.psqt,
        )
    }
}
