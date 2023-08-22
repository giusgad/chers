use super::Engine;
use crate::{
    consts::Info,
    search::consts::{SearchControl, SearchResult},
    uci::{consts::UciData, Uci},
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

    fn uci_command(&self, command: UciData) {
        match command {
            UciData::Uci => {
                // engine identifies and swtiches to uci mode
                Uci::output("id name Chers");
                Uci::output("id author Giuseppe Gadola");
                // NOTE: specify possible options here
                Uci::output("uciok");
            }

            UciData::IsReady => Uci::output("readyok"),
            UciData::Go(_) => self.search.send(SearchControl::Start),
            _ => (),
        }
    }

    fn search_report(&self, res: SearchResult) {
        dbg!(res);
    }
}
