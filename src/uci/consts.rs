#[derive(Debug)]
pub enum UciData {
    Uci,
    Debug(String),
    IsReady,
    SetOption(String),
    Register(String),
    AaaaNewGame,
    Position(String),
    Go(String),
    Stop,
    PonderHit,
    Quit,
}
