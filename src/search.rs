mod alpha_beta;
pub mod defs;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::{board::Board, defs::Info, moves::MoveGenerator};

use self::defs::{SearchControl, SearchResult};

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

            let mut board = board.lock().expect("Error locking board mutex");
            let depth = 12; // TODO: adaptive depth

            while !quit && !stop {
                let cmd = rx.recv().expect("Error in search receiving cmd");
                match cmd {
                    // TODO: implement start and stop functionality
                    SearchControl::Start => {
                        use rand::Rng;
                        let moves = mg.get_all_legal_moves(&board);
                        let mut rng = rand::thread_rng();
                        let i = rng.gen_range(0..moves.index);

                        report_tx.send(Info::Search(SearchResult::BestMove(moves.list[i])));

                        /* let a = Self::alpha_beta(&mut *board, &mg, depth, -25000, 25000);
                        println!("finished:{a}"); */
                    }
                    SearchControl::Stop => stop = true,
                    SearchControl::Quit => quit = true,
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
