mod alpha_beta;
pub mod defs;
mod draw;
mod iter_deep;
mod quiescence;
mod time;

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
    engine::{
        options::Options,
        transposition::{SearchData, TT},
    },
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
        tt: Arc<Mutex<TT<SearchData>>>,
        options: Arc<Mutex<Options>>,
    ) {
        let (tx, rx) = mpsc::channel::<SearchControl>();

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
                        tt_loads: 0,
                        mg: &mg,
                        time_control: search_time,
                        timer: None,
                        info: &mut SearchInfo::new(),
                        terminate: SearchTerminate::Nothing,
                        report_tx: &report_tx,
                        control_rx: &rx,
                        options: &options,
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
