use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crossbeam_channel::{Receiver, Sender};

use crate::{
    board::Board,
    defs::{ErrFatal, Info},
    engine::{
        options::Options,
        transposition::{SearchData, TT},
    },
    moves::{defs::Move, MoveGenerator},
};

pub const MAX_PLY: u8 = 128;

// Searchcontrol is used to receive signals from the gui
#[derive(Debug)]
pub enum SearchControl {
    Start(SearchTime),
    Stop,
    Quit,
}

// SearchTerminate is used by Search internally to determine how it should stop
#[derive(PartialEq, Debug)]
pub enum SearchTerminate {
    Stop,
    Quit,
    Nothing,
}

#[derive(Debug)]
pub enum SearchResult {
    BestMove(Move),
    Error,
}

// GameTime contains the information for the time from the whole game sent by the gui
#[derive(Debug, PartialEq, Default)]
pub struct GameTime {
    pub wtime: u128, // in milliseconds
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub moves_to_go: Option<u16>, // value not always provided
}

// Time modes Search can use
#[derive(Debug, PartialEq)]
pub enum SearchTime {
    Adaptive(GameTime),
    Depth(u8), // in plys
    Nodes(u64),
    MoveTime(u128), // milliseconds
    Infinite,
}

// info on the current state of the search
#[derive(Default)]
pub struct SearchInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub ply: u8,
    pub nodes: u64,
    pub allocated_time: u128,
}

// Refs that are used by the search algorithms and passed into recursion
pub struct SearchRefs {
    pub board: Arc<Mutex<Board>>,
    pub tt: Arc<Mutex<TT<SearchData>>>,
    pub mg: Arc<MoveGenerator>,
    pub time_control: SearchTime,
    pub info: Arc<Mutex<SearchInfo>>,
    pub timer: Arc<Mutex<Option<Instant>>>,
    pub terminate: Arc<Mutex<SearchTerminate>>,
    pub report_tx: Arc<Sender<Info>>,
    pub control_rx: Arc<Receiver<SearchControl>>,
    pub options: Arc<Mutex<Options>>,
}

impl SearchRefs {
    pub fn timer_start(&self) {
        *self.timer.lock().expect(ErrFatal::LOCK) = Some(Instant::now())
    }
    pub fn timer_elapsed(&self) -> u128 {
        match *self.timer.lock().expect(ErrFatal::LOCK) {
            Some(t) => t.elapsed().as_millis(),
            None => 0,
        }
    }
    pub fn stopped(&self) -> bool {
        *self.terminate.lock().expect(ErrFatal::LOCK) != SearchTerminate::Nothing
    }
}

impl std::fmt::Debug for SearchRefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchRefs")
            .field("time_control", &self.time_control)
            .field("terminate", &self.terminate)
            .finish()
    }
}
