use crate::{defs::ErrFatal, eval::defs::Eval, moves::defs::Move, uci::Uci};

use super::{
    defs::{SearchRefs, SearchResult, SearchTime},
    Search,
};

const WINDOW: i16 = 50;

impl Search {
    pub fn iterative_deepening(refs: &mut SearchRefs) -> SearchResult {
        refs.info.allocated_time = match &refs.time_control {
            SearchTime::Adaptive(_) => Self::calculate_time(refs),
            SearchTime::MoveTime(time) => *time,
            _ => 0,
        };

        let mut depth = 1;
        let mut pv: Vec<Move> = Vec::new();
        let mut best_move = Move::default();
        let mut stop = false;

        refs.timer_start();

        let (mut alpha, mut beta) = (-Eval::INF, Eval::INF);

        while !stop {
            refs.info.depth = depth;

            let eval = Self::alpha_beta(depth, alpha, beta, &mut pv, refs);

            // The aspiration window technique relies on the fact that likely the next iteration
            // will have similar result to the current one, so we can set up alpha and beta to
            // cutoff search branches earlier and increase performance
            if eval <= alpha || eval >= beta {
                // the search feel outside of the aspiration window, reset the values
                // and search again with full window
                alpha = -Eval::INF;
                beta = Eval::INF;
                // note that depth is not increased
                continue;
            }
            // set up the aspiration window for the next iteration
            alpha = eval - WINDOW;
            beta = eval + WINDOW;

            // update the stop condition before sending search info
            stop = refs.stopped();
            if !pv.is_empty() && !stop {
                // set the new best move and send stats to the gui
                best_move = pv[0];
                let hash_full = refs.tt.hash_full();
                Uci::search_info(refs, &pv, eval, hash_full);
            }

            if refs.options.lock().expect(ErrFatal::LOCK).early_stop
                && eval > Eval::CHECKMATE_TRESHOLD
            {
                // if a checkmate is found finish early
                stop = true;
                // TODO:if possible finish early when there is only one legal move
            }

            // increase depth for the next iteration
            depth += 1;
        }

        let null_move = Move::default();
        // return this and it will be sent to the main loop
        if best_move != null_move {
            SearchResult::BestMove(best_move)
        } else {
            SearchResult::Error
        }
    }
}
