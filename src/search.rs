mod alpha_beta;
pub mod defs;
mod draw;
mod iter_deep;
mod quiescence;
mod time;

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::Sender;

use crate::{
    board::Board,
    defs::{ErrFatal, Info},
    engine::{
        options::Options,
        transposition::{SearchData, TT},
    },
    moves::{defs::Move, MoveGenerator},
    search::defs::SearchTerminate,
};

use self::defs::{SearchControl, SearchInfo, SearchRefs, SearchTime, MAX_PLY};

#[derive(Default)]
pub struct Search {
    pub control_tx: Option<Sender<SearchControl>>, // control tx is used in the engine to send commands
    pub handle: Option<JoinHandle<()>>,
}

impl Search {
    pub fn init(
        &mut self,
        report_tx: Sender<Info>,
        board: Arc<Mutex<Board>>,
        mg: Arc<MoveGenerator>,
        tt: Arc<Mutex<TT<SearchData>>>,
        options: Arc<Mutex<Options>>,
    ) {
        let (tx, rx) = crossbeam_channel::unbounded();

        let h = thread::spawn(move || {
            let mut quit = false;
            let mut stop = true;

            while !quit {
                let cmd = rx.recv().expect(ErrFatal::RX_RECV);
                let mut search_time = SearchTime::Infinite;
                match cmd {
                    SearchControl::Start(time) => {
                        stop = false;
                        search_time = time;
                    }
                    SearchControl::Stop => stop = true,
                    SearchControl::Quit => quit = true,
                }
                if !stop && !quit {
                    let mut board = board.lock().expect(ErrFatal::LOCK);
                    let mut tt = tt.lock().expect(ErrFatal::LOCK);

                    let mut refs = SearchRefs {
                        board: &mut board,
                        tt: &mut tt,
                        killer_moves: [[Move::default(); 2]; MAX_PLY as usize],
                        mg: &mg,
                        time_control: search_time,
                        timer: None,
                        info: &mut SearchInfo::default(),
                        terminate: SearchTerminate::Nothing,
                        report_tx: &report_tx,
                        control_rx: &rx,
                        options: &options,
                    };

                    let res = Self::iterative_deepening(&mut refs);
                    quit = refs.terminate == SearchTerminate::Quit;
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
