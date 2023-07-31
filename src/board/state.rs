use crate::consts::{Color, Colors};

use super::consts::Castling;

pub struct State {
    pub active_color: Color,
    pub castling: u8,
    pub ep_square: Option<usize>,
    pub halfmove_count: u8,
    pub fullmove_count: u16,
    pub material: [u16; Colors::BOTH],
}

impl State {
    pub fn new() -> Self {
        Self {
            castling: Castling::ALL,
            active_color: Colors::WHITE,
            ep_square: None,
            halfmove_count: 0,
            fullmove_count: 0,
            material: [0; Colors::BOTH],
        }
    }
}

impl Into<String> for State {
    fn into(self) -> String {
        format!("active color: {}\ncastling: {}\nep square: {:?}\nhalfmove clock: {}\nfullmove count: {}\nmaterial: {:?}",
            self.active_color,
            self.castling,
            self.ep_square,
            self.halfmove_count,
            self.fullmove_count,
            self.material,
        )
    }
}
