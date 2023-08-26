use super::Engine;
use crate::{defs::Info, search::defs::SearchResult};
use std::sync::{mpsc, Arc};

impl Engine {
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel::<Info>();

        self.uci.init(tx.clone());
        self.search
            .init(tx, Arc::clone(&self.board), Arc::clone(&self.mg));

        while !self.quit {
            match rx.recv().expect("error in receiving info in main loop") {
                Info::Search(info) => self.search_report(info),
                Info::Uci(info) => self.uci_command(info),
            }
        }
    }

    fn search_report(&self, info: SearchResult) {
        dbg!(&info);
        let a = match info {
            SearchResult::BestMove(m) => self.board.lock().unwrap().make_move(m, &self.mg),
            _ => true,
        };
        println!("{}", self.board.lock().unwrap());
    }
}
