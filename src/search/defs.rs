use std::{sync::Arc, time::Instant};

use crate::{
    board::Board,
    moves::{defs::Move, MoveGenerator},
};

// Searchcontrol is used to receive signals from the gui
#[derive(Debug)]
pub enum SearchControl {
    Start(SearchTime),
    Stop,
    Quit,
    Nothing,
}

// SearchTerminate is used by Search internally to determine how it should stop
#[derive(PartialEq)]
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
    pub moves_to_go: u64,
}
impl GameTime {
    pub fn new() -> Self {
        Self {
            wtime: 0,
            btime: 0,
            winc: 0,
            binc: 0,
            moves_to_go: 0,
        }
    }
}

// Time modes Search can use
#[derive(Debug, PartialEq)]
pub enum SearchTime {
    Adaptive(GameTime),
    Depth(u64), // in plys
    Nodes(u64),
    MoveTime(u128), // milliseconds
    Infinite,
}

// Refs that are used by the search algorithms and passed into recursion
pub struct SearchRefs<'a> {
    pub board: &'a mut Board,
    pub mg: &'a Arc<MoveGenerator>,
    pub time: SearchTime,
    pub timer: Option<Instant>,
    pub terminate: SearchTerminate,
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
    pub fn timer_stop(&mut self) {
        self.timer = None;
    }
    pub fn stopped(&self) -> bool {
        self.terminate != SearchTerminate::Nothing
    }
}
