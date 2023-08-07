use super::consts::Move;
use crate::consts::MAX_LEGAL_MOVES;

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
}
