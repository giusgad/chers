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

use crate::{
    board::Board,
    defs::{ErrFatal, Info},
    moves::MoveGenerator,
    search::defs::SearchTerminate,
};

use self::defs::{SearchControl, SearchInfo, SearchRefs, SearchTime};

pub struct Search {
    pub control_tx: Option<Sender<SearchControl>>, // control tx is used in the engine to send commands
    pub handle: Option<JoinHandle<()>>,
}

impl Search {
    pub fn new() -> Self {
        Self {
            control_tx: None,
            handle: None,
        }
    }

    pub fn init(
        &mut self,
        report_tx: Sender<Info>,
        board: Arc<Mutex<Board>>,
        mg: Arc<MoveGenerator>,
    ) {
        let (tx, rx) = mpsc::channel::<SearchControl>();

        let h = thread::spawn(move || {
            let mut quit = false;
            let mut stop = false;

            while !quit {
                let cmd = rx.recv().expect(ErrFatal::RX_RECV);
                let mut search_time = SearchTime::Infinite;
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

                        if moves.len() == 0 {
                            report_tx.send(Info::Uci(crate::uci::defs::UciData::Quit));
                        } else {
                            report_tx.send(Info::Search(SearchResult::BestMove(moves[i])));
                        }*/
                        stop = false;
                        search_time = time;
                    }
                    SearchControl::Stop => stop = true,
                    SearchControl::Quit => quit = true,
                    SearchControl::Nothing => (),
                }
                if !stop && !quit {
                    let mut board = board.lock().expect(ErrFatal::LOCK);

                    let mut refs = SearchRefs {
                        board: &mut board,
                        mg: &mg,
                        time_control: search_time,
                        timer: None,
                        info: &mut SearchInfo::new(),
                        terminate: SearchTerminate::Nothing,
                        report_tx: &report_tx,
                    };

                    let res = Self::iterative_deepening(&mut refs);
                    report_tx.send(Info::Search(res)).expect(ErrFatal::TX_SEND);
                }
            }
        });

        self.control_tx = Some(tx);
        self.handle = Some(h);
    }

    pub fn send(&self, cmd: SearchControl) {
        match &self.control_tx {
            Some(tx) => tx.send(cmd).expect(ErrFatal::TX_SEND),
            None => panic!("No search tx"),
        }
    }
}
