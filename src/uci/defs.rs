#[derive(Debug)]
pub enum UciData {
    Uci,
    IsReady,
    NewGame,
    Position(String, Vec<String>), // fen string, moves vec
    Go(String),
    Stop,
    Quit,

    Error,
}
