use crate::{
    defs::{ErrFatal, Info},
    eval::defs::Eval,
    moves::defs::Move,
    uci::Uci,
};

use super::{
    defs::{SearchRefs, SearchResult, SearchTime},
    Search,
};

impl Search {
    pub fn iterative_deepening(refs: &mut SearchRefs) -> SearchResult {
        refs.info.allocated_time = match &refs.time_control {
            SearchTime::Adaptive(_) => Self::calculate_time(&refs),
            SearchTime::MoveTime(time) => *time,
            _ => 0,
        };

        let mut depth = 1;
        let mut pv: Vec<Move> = Vec::new();
        let mut best_move = Move { data: 0 };
        let mut stop = false;

        refs.timer_start();
        while !stop {
            refs.info.depth = depth;

            Self::alpha_beta(depth, -Eval::INF, Eval::INF, &mut pv, refs);

            // update the stop condition before sending search info
            stop = refs.stopped();

            if !pv.is_empty() && !stop {
                best_move = pv[0];
                Uci::search_info(&refs, &pv);
            }
            if pv.len() < depth as usize {
                // pv length is less than the current depth which means that the games forcibly
                // ends, so the search finishes early.
                stop = true;
            }

            depth += 1;
        }

        let null_move = Move { data: 0 };

        // return this and it will be sent to the main loop
        if best_move != null_move {
            SearchResult::BestMove(best_move)
        } else {
            SearchResult::Error
        }
    }
}
