use crate::{defs::ErrFatal, moves::defs::Move, search::defs::SearchRefs};

use super::Uci;

impl Uci {
    pub fn search_info(refs: &SearchRefs, moves: &[Move], eval: i16) {
        let moves: String = moves.iter().fold(String::new(), |mut s, m| {
            s.push_str(&format!("{}", m));
            s.push(' ');
            s
        });
        let mut nps = 0;
        let time = refs.timer_elapsed() as f64 / 1000f64;
        let info = refs.info.lock().expect(ErrFatal::LOCK);
        if time > 0f64 {
            nps = (info.nodes as f64 / time).round() as u64;
        }
        println!(
            "info depth {} seldepth {} score cp {} nodes {} nps {} hashfull {} time {} pv {}",
            info.depth,
            info.seldepth,
            eval,
            info.nodes,
            nps,
            refs.tt.lock().expect(ErrFatal::LOCK).hash_full(),
            refs.timer_elapsed(),
            moves
        );
    }
}
