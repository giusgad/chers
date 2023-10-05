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
    moves::MoveGenerator,
    search::defs::SearchTerminate,
};

use self::defs::{SearchControl, SearchInfo, SearchRefs, SearchTime};

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
                    let refs = SearchRefs {
                        board: Arc::clone(&board),
                        tt: Arc::clone(&tt),
                        time_control: search_time,
                        timer: Arc::new(Mutex::new(None)),
                        info: Arc::new(Mutex::new(SearchInfo::default())),
                        terminate: Arc::new(Mutex::new(SearchTerminate::Nothing)),
                        report_tx: Arc::new(report_tx.clone()),
                        control_rx: Arc::new(rx.clone()),
                        mg: Arc::clone(&mg),
                        options: Arc::clone(&options),
                    };

                    let res = Self::iterative_deepening(&refs);
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
