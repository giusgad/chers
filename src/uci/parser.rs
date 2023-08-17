use super::{consts::UciData, Uci};

impl Uci {
    pub fn commands_from_string(s: String) -> Vec<UciData> {
        let v = Vec::new();
        let s: Vec<&str> = s.split_whitespace().collect();
        v
    }
}
