mod thread;

use crate::{board::Board, moves::MoveGenerator};

pub struct Engine {
    board: Board,
    movegen: MoveGenerator,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            movegen: MoveGenerator::new(),
        }
    }
}
