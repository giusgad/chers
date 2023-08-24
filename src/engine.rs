mod commands;
pub mod main_loop;

use std::sync::{Arc, Mutex};

use crate::{board::Board, moves::MoveGenerator, search::Search, uci::Uci};

pub struct Engine {
    pub board: Arc<Mutex<Board>>,
    mg: Arc<MoveGenerator>,
    search: Search,
    uci: Uci,
    stop: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(Board::new())),
            mg: Arc::new(MoveGenerator::new()),
            uci: Uci::new(),
            search: Search::new(),
            stop: false,
        }
    }
}
