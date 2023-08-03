use crate::{consts::MAX_MOVES, search::consts::Move};

pub struct History {
    moves: [Move; MAX_MOVES],
}

impl History {
    pub fn new() -> Self {
        Self {
            moves: [Move { data: 0 }; MAX_MOVES],
        }
    }
}
