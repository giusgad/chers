mod commands;
pub mod main_loop;
mod position;
mod transposition;

use std::sync::{Arc, Mutex};

use self::transposition::TT;
use crate::{
    board::Board,
    defs::Options,
    moves::MoveGenerator,
    search::{defs::SearchControl, Search},
    uci::Uci,
};

pub struct Engine {
    options: Options,
    pub board: Arc<Mutex<Board>>,
    mg: Arc<MoveGenerator>,
    tt: Arc<Mutex<TT>>,
    search: Search,
    uci: Uci,
    quit: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            options: Options::new(),
            board: Arc::new(Mutex::new(Board::new())),
            mg: Arc::new(MoveGenerator::new()),
            uci: Uci::new(),
            tt: Arc::new(Mutex::new(TT::new())),
            search: Search::new(),
            quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.search.send(SearchControl::Quit);
        self.quit = true;
    }
}
