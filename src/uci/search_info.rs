use crate::{moves::defs::Move, search::defs::SearchRefs};

use super::Uci;

impl Uci {
    pub fn search_info(refs: &SearchRefs, moves: &Vec<Move>, eval: i16) {
        let moves: String = moves.iter().fold(String::new(), |mut s, m| {
            s.push_str(&format!("{}", m));
            s.push(' ');
            s
        });
        println!(
            "info depth {} seldepth {} score cp {} nodes {} time {} pv {}",
            refs.info.depth,
            refs.info.seldepth,
            eval,
            refs.info.nodes,
            refs.timer_elapsed(),
            moves
        );
    }
}
