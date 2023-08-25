use crate::search::defs::SearchTime;

// UCI commands from the gui to which the engine responds:
#[derive(Debug)]
pub enum UciData {
    Uci,
    IsReady,
    NewGame,
    Position(String, Vec<String>), // fen string, moves vec
    Go(SearchTime),
    Stop,
    Quit,

    Error,
}
