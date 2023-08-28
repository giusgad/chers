use crate::{
    defs::{ErrFatal, Info},
    eval::defs::Eval,
    moves::defs::Move,
    uci::Uci,
};

use super::{
    defs::{GameTime, SearchRefs, SearchResult, SearchTime},
    Search,
};

impl Search {
    pub fn iterative_deepening(refs: &mut SearchRefs) -> SearchResult {
        /* match &refs.time_control {
            SearchTime::Adaptive(time) => refs.info.allocated_time = Self::calculate_time(time),
            SearchTime::MoveTime(time) => refs.info.allocated_time = *time,
            _ => (),
        } */
        match &refs.time_control {
            SearchTime::Adaptive(_) => refs.time_control = SearchTime::Depth(6),
            SearchTime::MoveTime(time) => refs.info.allocated_time = *time,
            _ => (),
        }

        let mut depth = 1;
        let mut pv: Vec<Move> = Vec::new();
        let mut best_move = Move { data: 0 };
        let mut stop = false;

        refs.timer_start();
        while !stop {
            refs.info.depth = depth;

            Self::alpha_beta(depth, -Eval::INF, Eval::INF, &mut pv, refs);

            if !pv.is_empty() {
                best_move = pv[0];
                Uci::search_info(&refs, &pv);
            }

            depth += 1;
            stop = refs.stopped();
        }

        dbg!(refs.timer_elapsed());

        let null_move = Move { data: 0 };
        if best_move != null_move {
            refs.report_tx
                .send(Info::Search(SearchResult::BestMove(best_move)))
                .expect(ErrFatal::TX_SEND);
        }

        SearchResult::Error
    }
}

impl Search {
    fn calculate_time(time: &GameTime) -> u128 {
        0
    }
}
