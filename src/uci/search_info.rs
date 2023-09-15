use crate::{moves::defs::Move, search::defs::SearchRefs};

use super::Uci;

impl Uci {
    pub fn search_info(refs: &SearchRefs, moves: &Vec<Move>, eval: i16, hash_full: u16) {
        let moves: String = moves.iter().fold(String::new(), |mut s, m| {
            s.push_str(&format!("{}", m));
            s.push(' ');
            s
        });
        let mut nps = 0;
        let time = refs.timer_elapsed() as f64 / 1000f64;
        if time > 0f64 {
            nps = (refs.info.nodes as f64 / time).round() as u64;
        }
        println!(
            "info depth {} seldepth {} score cp {} nodes {} nps {} hashfull {} time {} pv {}",
            refs.info.depth,
            refs.info.seldepth,
            eval,
            refs.info.nodes,
            nps,
            hash_full,
            refs.timer_elapsed(),
            moves
        );
    }
}
