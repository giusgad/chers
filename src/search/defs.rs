use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crossbeam_channel::{Receiver, Sender};

use crate::{
    board::Board,
    defs::{Colors, Info, NrOf},
    engine::{
        options::Options,
        transposition::{SearchData, TT},
    },
    moves::{defs::Move, MoveGenerator},
};

pub const MAX_PLY: u8 = 128;
pub const MAX_DEPTH: u8 = 99;

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

pub type HistoryHeuristic = [[[u16; NrOf::SQUARES]; NrOf::SQUARES]; Colors::BOTH];

// Refs that are used by the search algorithms and passed into recursion
pub struct SearchRefs<'a> {
    pub board: &'a mut Board,
    pub tt: &'a mut TT<SearchData>,
    pub killer_moves: [[Move; 2]; MAX_PLY as usize],
    pub history_heuristic: HistoryHeuristic,
    pub mg: &'a Arc<MoveGenerator>,
    pub time_control: SearchTime,
    pub info: &'a mut SearchInfo,
    pub timer: Option<Instant>,
    pub terminate: SearchTerminate,
    pub report_tx: &'a Sender<Info>,
    pub control_rx: &'a Receiver<SearchControl>,
    pub options: &'a Arc<Mutex<Options>>,
}

impl SearchRefs<'_> {
    pub fn timer_start(&mut self) {
        self.timer = Some(Instant::now());
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
