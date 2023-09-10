use super::Engine;
use crate::{
    defs::EngineOption,
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

            UciData::NewGame => (), // TODO: clear the tt
            UciData::IsReady => Uci::output("readyok"),
            UciData::Go(time) => self.search.send(SearchControl::Start(time)),
            UciData::Position(fen, moves) => {
                self.setup_position(fen, moves);
            }
            UciData::Option(opt) => self.set_option(opt),

            UciData::Stop => self.search.send(SearchControl::Stop),

            UciData::Dbg(s) => match s.as_str() {
                "draw" => {
                    dbg!(Search::is_draw(&self.board.lock().unwrap()));
                }
                "endgame" => {
                    dbg!(self.board.lock().unwrap().is_endgame());
                }
                "board" => println!("{}", self.board.lock().unwrap()),
                _ => {
                    dbg!(self.board.lock().unwrap().state);
                }
            },

            UciData::Quit => self.quit(), // TODO: close threads with handles
            UciData::Error => (),
        }
    }
}

impl Engine {
    fn set_option(&mut self, opt: EngineOption) {
        use EngineOption::*;
        match opt {
            HashSize(size) => self.options.hash_size = size,
        }
    }
}
