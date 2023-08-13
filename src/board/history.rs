use crate::consts::MAX_MOVE_COUNT;

use super::state::State;

// TODO: hold state instead of moves
pub struct History {
    moves: [State; MAX_MOVE_COUNT],
    current: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            moves: [State::new(); MAX_MOVE_COUNT],
            current: 0,
        }
    }
    pub fn push(&mut self, s: State) {
        self.moves[self.current] = s;
        self.current += 1;
    }
    pub fn pop(&mut self) -> State {
        self.current -= 1;
        self.moves[self.current]
    }
}
