mod commands;
pub mod main_loop;
mod position;
mod transposition_table;

use std::sync::{Arc, Mutex};

use crate::{
    board::Board,
    moves::MoveGenerator,
    search::{defs::SearchControl, Search},
    uci::Uci,
};

pub struct Engine {
    pub board: Arc<Mutex<Board>>,
    mg: Arc<MoveGenerator>,
    search: Search,
    uci: Uci,
    quit: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(Board::new())),
            mg: Arc::new(MoveGenerator::new()),
            uci: Uci::new(),
            search: Search::new(),
            quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.search.send(SearchControl::Quit);
        self.quit = true;
    }
}
