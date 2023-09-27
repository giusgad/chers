use crate::{
    engine::options::EngineOption,
    search::defs::{GameTime, SearchTime},
};

use super::{defs::UciData, Uci};

impl Uci {
    pub fn commands_from_string(s: String) -> UciData {
        let s = s.trim(); // remove the \n
        match s {
            cmd if cmd == "uci" => UciData::Uci,
            cmd if cmd == "isready" => UciData::IsReady,
            cmd if cmd == "newgame" => UciData::NewGame,
            cmd if cmd == "stop" => UciData::Stop,
            cmd if cmd == "quit" => UciData::Quit,

            cmd if cmd.starts_with("position") => Self::parse_position(cmd),
            cmd if cmd.starts_with("go") => UciData::Go(Self::parse_go(cmd)),
            cmd if cmd.starts_with("setoption") => Self::parse_option(cmd),

            cmd if cmd.starts_with("dbg") => {
                UciData::Dbg(cmd.strip_prefix("dbg").unwrap_or("").trim().to_string())
            }
            _ => UciData::Error,
        }
    }
}

enum PosToken {
    Fen,
    Moves,
    None,
}
enum GoToken {
    WTime,
    BTime,
    WInc,
    BInc,
    MovesToGo,
    Depth,
    Nodes,
    MoveTime,
    None,
}
enum OptToken {
    Name,
    Value,
    None,
}
impl Uci {
    fn parse_position(cmd: &str) -> UciData {
        let cmd: Vec<&str> = cmd.split_whitespace().collect();

        let mut moves: Vec<String> = Vec::new();
        let mut token = PosToken::None;
        let mut fen = String::new();

        for part in cmd.iter() {
            match *part {
                "fen" | "position" => token = PosToken::Fen,
                "moves" => token = PosToken::Moves,
                s => match token {
                    PosToken::Fen => {
                        fen.push_str(s);
                        fen.push(' ')
                    }
                    PosToken::Moves => moves.push(s.to_string()),
                    PosToken::None => (),
                },
            }
        }

        UciData::Position(fen, moves)
    }

    fn parse_go(cmd: &str) -> SearchTime {
        let cmd: Vec<&str> = cmd.split_whitespace().collect();

        let mut token = GoToken::None;
        let mut time = GameTime::default();
        let mut depth = 0;
        let mut nodes = 0;
        let mut move_time = 0;

        for part in cmd.iter() {
            match *part {
                "wtime" => token = GoToken::WTime,
                "btime" => token = GoToken::BTime,
                "winc" => token = GoToken::WInc,
                "binc" => token = GoToken::BInc,
                "depth" => token = GoToken::Depth,
                "nodes" => token = GoToken::Nodes,
                "movestogo" => token = GoToken::MovesToGo,
                "movetime" => token = GoToken::MoveTime,

                "infinite" => return SearchTime::Infinite,

                t => match token {
                    GoToken::WTime => time.wtime = t.parse().unwrap_or(0),
                    GoToken::BTime => time.btime = t.parse().unwrap_or(0),
                    GoToken::WInc => time.winc = t.parse().unwrap_or(0),
                    GoToken::BInc => time.binc = t.parse().unwrap_or(0),
                    GoToken::MovesToGo => time.moves_to_go = t.parse().ok(),
                    GoToken::Depth => depth = t.parse().unwrap_or(0),
                    GoToken::Nodes => nodes = t.parse().unwrap_or(0),
                    GoToken::MoveTime => move_time = t.parse().unwrap_or(0),
                    GoToken::None => (),
                },
            }
        }

        if time.wtime > 0 || time.btime > 0 {
            return SearchTime::Adaptive(time);
        } else if depth > 0 {
            return SearchTime::Depth(depth);
        } else if nodes > 0 {
            return SearchTime::Nodes(nodes);
        } else if move_time > 0 {
            return SearchTime::MoveTime(move_time);
        }

        SearchTime::Infinite
    }
    fn parse_option(cmd: &str) -> UciData {
        let cmd: Vec<&str> = cmd.split_whitespace().collect();

        let mut token = OptToken::None;
        let mut name = "";
        let mut val = "";

        for part in cmd.iter() {
            match *part {
                "name" => token = OptToken::Name,
                "value" => token = OptToken::Value,
                t => match token {
                    OptToken::Name => name = t,
                    OptToken::Value => val = t,
                    OptToken::None => (),
                },
            }
        }

        match name {
            "Hash" => UciData::Option(EngineOption::Hash(val.parse().ok())),
            "EarlyStop" => UciData::Option(EngineOption::EarlyStop(val.to_lowercase() == "true")),
            "DbgUnicode" => UciData::Option(EngineOption::DbgUnicode(val.to_lowercase() == "true")),
            _ => UciData::Error,
        }
    }
}
