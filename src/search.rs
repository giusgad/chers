mod alpha_beta;
pub mod defs;
mod iter_deep;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::{board::Board, defs::Info, moves::MoveGenerator, search::defs::SearchTerminate};

use self::defs::{SearchControl, SearchRefs, SearchResult, SearchTime};

pub struct Search {
    pub control_tx: Option<Sender<SearchControl>>, // control tx is used in the engine to send commands
}

impl Search {
    pub fn new() -> Self {
        Self { control_tx: None }
    }

    pub fn init(
        &mut self,
        report_tx: Sender<Info>,
        board: Arc<Mutex<Board>>,
        mg: Arc<MoveGenerator>,
    ) {
        let (tx, rx) = mpsc::channel::<SearchControl>();

        thread::spawn(move || {
            let mut quit = false;
            let mut stop = false;

            while !quit {
                let cmd = rx.recv().expect("Error in search receiving cmd");
                let mut search_time = SearchTime::Infinite;
                dbg!(&cmd);
                match cmd {
                    SearchControl::Start(time) => {
                        /* use rand::Rng;
                        let mut board = board.lock().expect("Error locking board mutex");
                        let mut moves: Vec<crate::moves::defs::Move> =
                            mg.get_all_legal_moves(&board).iter().map(|s| *s).collect();
                        let mut rng = rand::thread_rng();
                        let mut i = 0;
                        while moves.len() > 0 {
                            i = rng.gen_range(0..moves.len());
                            if board.make_move(moves[i], &mg) {
                                board.unmake();
                                break;
                            } else {
                                moves.remove(i);
                            }
                        }

                        // if moves.len() == 0 {
                        //     report_tx.send(Info::Uci(crate::uci::defs::UciData::Quit));
                        // } else {
                        report_tx.send(Info::Search(SearchResult::BestMove(moves[i])));
                        // } */

                        search_time = time;
                        stop = false;
                    }
                    SearchControl::Stop => stop = true,
                    SearchControl::Quit => quit = true,
                    SearchControl::Nothing => (),
                }
                if !quit && !stop {
                    let mut board = board.lock().expect("Error locking board mutex");

                    let refs = SearchRefs {
                        board: &mut board,
                        mg: &mg,
                        time: search_time,
                        timer: None,
                        terminate: SearchTerminate::Nothing,
                    };

                    let res = Self::iterative_deepening(&refs);
                }
            }
        });

        self.control_tx = Some(tx);
    }

    pub fn send(&self, cmd: SearchControl) {
        match &self.control_tx {
            Some(tx) => tx.send(cmd).expect("Error sending command to search"),
            None => panic!("No search tx"),
        }
    }
}
