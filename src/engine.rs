mod commands;
pub mod main_loop;
pub mod options;
mod position;
pub mod transposition;

use std::sync::{Arc, Mutex};

use self::{
    options::Options,
    transposition::{SearchData, TT},
};
use crate::{
    board::Board,
    moves::MoveGenerator,
    search::{defs::SearchControl, Search},
    uci::Uci,
};

pub struct Engine {
    options: Arc<Mutex<Options>>,
    pub board: Arc<Mutex<Board>>,
    mg: Arc<MoveGenerator>,
    tt: Arc<Mutex<TT<SearchData>>>,
    search: Search,
    uci: Uci,
    quit: bool,
}

impl Engine {
    pub fn new() -> Self {
        let options = Options::default();
        let tt = TT::new(options.hash_size);
        let mut mg = MoveGenerator::default();
        mg.init();
        Self {
            options: Arc::new(Mutex::new(options)),
            board: Arc::new(Mutex::new(Board::new())),
            mg: Arc::new(mg),
            uci: Uci::default(),
            tt: Arc::new(Mutex::new(tt)),
            search: Search::default(),
            quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.search.send(SearchControl::Quit);
        self.quit = true;
    }
}
