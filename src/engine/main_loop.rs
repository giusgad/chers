use super::Engine;
use crate::{
    defs::{ErrFatal, Info},
    search::defs::SearchResult,
    uci::Uci,
};
use std::sync::Arc;

impl Engine {
    pub fn start(&mut self) {
        let (tx, rx) = crossbeam_channel::unbounded();

        self.uci.init(tx.clone());
        self.search.init(
            tx,
            Arc::clone(&self.board),
            Arc::clone(&self.mg),
            Arc::clone(&self.tt),
            Arc::clone(&self.options),
        );

        while !self.quit {
            match rx.recv().expect(ErrFatal::RX_RECV) {
                Info::Search(info) => Self::search_report(&info),
                Info::Uci(info) => self.uci_command(info),
            }
        }

        // main loop is finished, join the threads to quit
        if let Some(h) = self.search.handle.take() {
            h.join().expect(ErrFatal::THREAD_JOIN);
        }
        if let Some(h) = self.uci.handle.take() {
            h.join().expect(ErrFatal::THREAD_JOIN);
        }
    }

    fn search_report(info: &SearchResult) {
        match info {
            SearchResult::BestMove(m) => Uci::output(format!("bestmove {m}")),
            SearchResult::Error => Uci::output_err("Something went wrong with the search"),
        }
    }
}
