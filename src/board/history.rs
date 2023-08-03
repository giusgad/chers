use crate::{consts::MAX_MOVES, moves::consts::Move};

// TODO: hold state instead of moves
pub struct History {
    moves: [Move; MAX_MOVES],
    current: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            moves: [Move { data: 0 }; MAX_MOVES],
            current: 0,
        }
    }
    pub fn push(&mut self, m: Move) {
        self.moves[self.current] = m;
        self.current += 1;
    }
    pub fn pop(&mut self) -> Move {
        self.current -= 1;
        self.moves[self.current]
    }
}
