use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    time::Instant,
};

use crate::{
    board::Board,
    defs::Info,
    engine::transposition::TT,
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
#[derive(Debug, PartialEq)]
pub struct GameTime {
    pub wtime: u128, // in milliseconds
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub moves_to_go: Option<u16>, // value not always provided
}
impl GameTime {
    pub fn new() -> Self {
        Self {
            wtime: 0,
            btime: 0,
            winc: 0,
            binc: 0,
            moves_to_go: None,
        }
    }
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
pub struct SearchInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub ply: u8,
    pub nodes: u64,
    pub allocated_time: u128,
}
impl SearchInfo {
    pub fn new() -> Self {
        Self {
            depth: 0,
            seldepth: 0,
            ply: 0,
            nodes: 0,
            allocated_time: 0,
        }
    }
}

// Refs that are used by the search algorithms and passed into recursion
pub struct SearchRefs<'a> {
    pub board: &'a mut Board,
    pub tt: &'a mut TT,
    pub tt_loads: u64,
    pub mg: &'a Arc<MoveGenerator>,
    pub time_control: SearchTime,
    pub info: &'a mut SearchInfo,
    pub timer: Option<Instant>,
    pub terminate: SearchTerminate,
    pub report_tx: &'a Sender<Info>,
    pub control_rx: &'a Receiver<SearchControl>,
}

impl SearchRefs<'_> {
    pub fn timer_start(&mut self) {
        self.timer = Some(Instant::now())
    }
    pub fn timer_elapsed(&self) -> u128 {
        match self.timer {
            Some(t) => t.elapsed().as_millis(),
            None => 0,
        }
    }
    pub fn stopped(&self) -> bool {
        self.terminate != SearchTerminate::Nothing
    }
}

impl std::fmt::Debug for SearchRefs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchRefs")
            .field("time_control", &self.time_control)
            .field("terminate", &self.terminate)
            .finish()
    }
}
