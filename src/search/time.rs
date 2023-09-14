use super::{
    defs::{SearchRefs, SearchTime},
    Search,
};
use crate::defs::Colors;

const GAME_MOVES: u16 = 45;
const EXTRA_MOVES: u16 = 5;
const LOW_TIME: u128 = 5000;
const CRIT_TIME: u128 = 1000;

impl Search {
    pub fn calculate_time(refs: &SearchRefs) -> u128 {
        let gt = match &refs.time_control {
            SearchTime::Adaptive(gt) => gt,
            _ => panic!(),
        };
        let (time, inc) = match refs.board.state.active_color {
            Colors::WHITE => (gt.wtime, gt.winc),
            Colors::BLACK => (gt.btime, gt.binc),
            _ => panic!("Invalid active color"),
        };
        let moves = Self::moves_to_go(refs);

        if time < LOW_TIME && time > CRIT_TIME {
            CRIT_TIME
        } else if time < CRIT_TIME {
            200
        } else {
            (time / moves as u128) + inc
        }
    }

    fn moves_to_go(refs: &SearchRefs) -> u16 {
        let moves = match &refs.time_control {
            SearchTime::Adaptive(gt) => gt.moves_to_go,
            _ => panic!(),
        };
        if let Some(moves) = moves {
            return moves;
        } else {
            let tot_moves = refs.board.state.fullmove_count;
            GAME_MOVES.saturating_sub(tot_moves) + EXTRA_MOVES
        }
    }
}
