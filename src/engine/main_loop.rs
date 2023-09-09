use super::Engine;
use crate::{
    defs::{ErrFatal, Info},
    search::defs::SearchResult,
    uci::Uci,
};
use std::sync::{mpsc, Arc};

impl Engine {
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel::<Info>();

        self.uci.init(tx.clone());
        self.search
            .init(tx, Arc::clone(&self.board), Arc::clone(&self.mg));

        while !self.quit {
            match rx.recv().expect(ErrFatal::RX_RECV) {
                Info::Search(info) => self.search_report(info),
                Info::Uci(info) => self.uci_command(info),
            }
        }

        if let Some(h) = self.search.handle.take() {
            h.join().expect(ErrFatal::THREAD_JOIN);
        }
        if let Some(h) = self.uci.handle.take() {
            h.join().expect(ErrFatal::THREAD_JOIN);
        }
    }

    fn search_report(&self, info: SearchResult) {
        // dbg!(&info);
        match info {
            SearchResult::BestMove(m) => Uci::output(format!("bestmove {}", m)),
            _ => (),
        }
        // println!("{}", self.board.lock().unwrap());
    }
}
