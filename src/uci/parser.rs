use crate::search::defs::{GameTime, SearchTime};

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
            cmd if cmd.starts_with("go") => UciData::Go(Self::parse_go(cmd)),
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
impl Uci {
    fn parse_position(cmd: &str) -> UciData {
        let cmd: Vec<&str> = cmd.split_whitespace().collect();
        let mut cmd = cmd.iter();

        let mut moves: Vec<String> = Vec::new();
        let mut token = PosToken::None;
        let mut fen = String::new();

        while let Some(part) = cmd.next() {
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
        let mut cmd = cmd.iter();

        let mut token = GoToken::None;
        let mut time = GameTime::new();
        let mut depth = 0;
        let mut nodes = 0;
        let mut move_time = 0;

        while let Some(part) = cmd.next() {
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
                    GoToken::MovesToGo => time.moves_to_go = t.parse().unwrap_or(0),
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
}
