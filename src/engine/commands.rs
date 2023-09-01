use super::Engine;
use crate::{
    search::{defs::SearchControl, Search},
    uci::{defs::UciData, Uci},
};

impl Engine {
    pub fn uci_command(&mut self, command: UciData) {
        match command {
            UciData::Uci => {
                // engine identifies and swtiches to uci mode
                Uci::output("id name Chers");
                Uci::output("id author Giuseppe Gadola");
                // NOTE: specify possible options here
                Uci::output("uciok");
            }

            UciData::IsReady => Uci::output("readyok"),
            UciData::Go(time) => self.search.send(SearchControl::Start(time)),
            UciData::Position(fen, moves) => {
                self.setup_position(fen, moves);
            }

            UciData::Stop => self.search.send(SearchControl::Stop),

            UciData::Dbg(s) => {
                if s == "draw" {
                    dbg!(Search::is_draw(&self.board.lock().unwrap()));
                } else {
                    dbg!(self.board.lock().unwrap().state);
                }
            }
            UciData::PrintBoard => {
                println!("{}", self.board.lock().unwrap())
            }

            UciData::Quit => self.quit(), // TODO: close threads with handles
            _ => (),
        }
    }
}
