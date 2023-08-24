use super::{defs::UciData, Uci};

impl Uci {
    pub fn commands_from_string(s: String) -> UciData {
        let s = s.trim_end(); // remove the \n
        match s {
            cmd if cmd == "uci" => UciData::Uci,
            cmd if cmd == "isready" => UciData::IsReady,
            cmd if cmd == "newgame" => UciData::NewGame,
            cmd if cmd == "stop" => UciData::Stop,
            cmd if cmd == "quit" => UciData::Quit,

            cmd if cmd.starts_with("position") => Self::parse_position(cmd),
            cmd if cmd.starts_with("go") => Self::parse_go(cmd),
            _ => UciData::Error,
        }
    }
}

enum PosTokens {
    Fen,
    Moves,
    None,
}
impl Uci {
    fn parse_position(cmd: &str) -> UciData {
        let cmd: Vec<&str> = cmd.split_whitespace().collect();
        let mut cmd = cmd.iter();

        let mut moves: Vec<String> = Vec::new();
        let mut token = PosTokens::None;
        let mut fen_str = "";

        while let Some(part) = cmd.next() {
            match *part {
                "fen" | "position" => token = PosTokens::Fen,
                "moves" => token = PosTokens::Moves,
                s => match token {
                    PosTokens::Fen => fen_str = s,
                    PosTokens::Moves => moves.push(s.to_string()),
                    PosTokens::None => (),
                },
            }
        }

        UciData::Position(fen_str.to_string(), moves)
    }

    fn parse_go(cmd: &str) -> UciData {
        UciData::Go(String::new())
    }
}
