use std::ops::Deref;

use crate::defs::MAX_MOVE_COUNT;

use super::state::State;

pub struct History {
    list: [State; MAX_MOVE_COUNT],
    current: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            list: [State::new(); MAX_MOVE_COUNT],
            current: 0,
        }
    }
    pub fn push(&mut self, s: State) {
        self.list[self.current] = s;
        self.current += 1;
    }
    pub fn pop(&mut self) -> State {
        self.current -= 1;
        self.list[self.current]
    }
}

impl Deref for History {
    type Target = [State];

    fn deref(&self) -> &Self::Target {
        &self.list[..self.current]
    }
}
