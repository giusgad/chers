use crate::search::defs::SearchTime;

// UCI commands from the gui to which the engine responds:
#[derive(Debug, PartialEq)]
pub enum UciData {
    Uci,
    IsReady,
    NewGame,
    Position(String, Vec<String>), // fen string, moves vec
    Go(SearchTime),
    Stop,
    Quit,

    Dbg(String), // WARN: custom command for debug (not uci)
    PrintBoard,

    Error,
}
