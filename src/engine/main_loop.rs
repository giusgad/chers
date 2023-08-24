use super::Engine;
use crate::{
    defs::Info,
    search::defs::{SearchControl, SearchResult},
    uci::{defs::UciData, Uci},
};
use std::sync::{mpsc, Arc};

impl Engine {
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel::<Info>();

        self.uci.init(tx.clone());
        self.search
            .init(tx, Arc::clone(&self.board), Arc::clone(&self.mg));

        while !self.stop {
            match rx.recv().expect("error in receiving info in main loop") {
                Info::Search(info) => self.search_report(info),
                Info::Uci(info) => self.uci_command(info),
            }
        }
    }

    fn search_report(&self, res: SearchResult) {
        dbg!(res);
    }
}
