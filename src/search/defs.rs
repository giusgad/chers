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
