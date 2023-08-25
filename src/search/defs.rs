use crate::moves::defs::Move;

#[derive(Debug)]
pub enum SearchControl {
    Start,
    Stop,
    Quit,
}

#[derive(Debug)]
pub enum SearchResult {
    BestMove(Move),
    Error,
}

// time in milliseconds
#[derive(Debug)]
pub struct GameTime {
    pub wtime: u128,
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

// search time limits
// TODO: maybe change units
#[derive(Debug)]
pub enum SearchTime {
    Adaptive(GameTime),
    Depth(u64), // in plys
    Nodes(u64),
    MoveTime(u128), // milliseconds
    Infinite,
}
