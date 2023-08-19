use super::{consts::UciData, Uci};

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

impl Uci {
    fn parse_position(cmd: &str) -> UciData {
        todo!();
    }

    fn parse_go(cmd: &str) -> UciData {
        todo!();
    }
}
