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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        defs::{Info, START_FEN},
        engine::Engine,
        search::defs::{SearchControl, SearchResult, SearchTime},
    };

    #[test]
    fn full_search() {
        let fens = [
            START_FEN,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        ];
        let mut engine = Engine::new();
        for fen in fens {
            do_test(fen, &mut engine);
        }
    }
    fn do_test(fen: &str, engine: &mut Engine) {
        let (tx, rx) = crossbeam_channel::unbounded();
        engine.search.init(
            tx,
            Arc::clone(&engine.board),
            Arc::clone(&engine.mg),
            Arc::clone(&engine.tt),
            Arc::clone(&engine.options),
        );
        engine.board.lock().unwrap().read_fen(fen).unwrap();
        engine
            .search
            .send(SearchControl::Start(SearchTime::Depth(5)));
        let res = rx.recv().unwrap();
        engine.quit();
        match res {
            Info::Search(SearchResult::BestMove(_)) => (),
            _ => panic!(),
        };
    }
}
