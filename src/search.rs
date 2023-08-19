mod alpha_beta;
pub mod consts;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::{board::Board, moves::MoveGenerator};

use self::consts::SearchControl;

pub struct Search {
    pub control_tx: Option<Sender<SearchControl>>, // control tx is used in the engine to send commands
}

impl Search {
    pub fn new() -> Self {
        Self { control_tx: None }
    }

    pub fn init(&mut self, board: Arc<Mutex<Board>>, mg: Arc<MoveGenerator>) {
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
                        let a = Self::alpha_beta(&mut *board, &mg, depth, -25000, 25000);
                        println!("finished:{a}");
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
