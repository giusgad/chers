pub mod main_loop;

use crate::{board::Board, moves::MoveGenerator, uci::Uci};

pub struct Engine {
    board: Board,
    mg: MoveGenerator,
    uci: Uci,
    stop: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            mg: MoveGenerator::new(),
            uci: Uci::new(),
            stop: false,
        }
    }
}
