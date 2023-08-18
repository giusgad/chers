#[derive(Debug)]
pub enum UciData {
    Uci,
    IsReady,
    NewGame,
    Position(String),
    Go(String),
    Stop,
    Quit,

    Error,
}
