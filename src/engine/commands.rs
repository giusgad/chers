use super::Engine;
use crate::{
    search::defs::SearchControl,
    uci::{defs::UciData, Uci},
};

impl Engine {
    pub fn uci_command(&self, command: UciData) {
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
}
