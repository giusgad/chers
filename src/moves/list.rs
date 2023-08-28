use super::defs::{Move, MoveType};
use crate::defs::MAX_LEGAL_MOVES;

pub struct MoveList {
    pub list: [Move; MAX_LEGAL_MOVES as usize],
    pub index: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            list: [Move { data: 0 }; MAX_LEGAL_MOVES as usize],
            index: 0,
        }
    }

    pub fn push(&mut self, m: Move) {
        self.list[self.index] = m;
        self.index += 1;
    }

    // reorder the moves for a faster search
    pub fn reorder(&mut self) {
        let mut top = 0;
        for i in 0..self.index {
            let m = &self.list[i];
            if m.move_type() == MoveType::Capture {
                (self.list[top], self.list[i]) = (self.list[i], self.list[top]);
                top += 1;
            }
        }
    }
}

impl std::ops::Deref for MoveList {
    type Target = [Move];

    fn deref(&self) -> &Self::Target {
        &self.list[..self.index]
    }
}
