use std::thread;

use super::{
    defs::{SearchRefs, MAX_PLY},
    Search,
};
use crate::{
    defs::ErrFatal,
    engine::transposition::{EvalType, SearchData},
    eval::{defs::Eval, evaluate},
    moves::defs::Move,
    search::defs::{SearchControl, SearchTerminate},
};

impl Search {
    pub fn alpha_beta(
        mut depth: u8,
        mut alpha: i16,
        beta: i16,
        pv: &mut Vec<Move>,
        refs: &SearchRefs,
    ) -> i16 {
        // check if the search has to stop
        // only perform the check every 2048 nodes for efficency,
        // since the calculation of the elapsed time is expensive
        if refs.info.lock().expect(ErrFatal::LOCK).nodes & 2048 == 0 {
            Self::check_termination(refs);
        }

        if refs.stopped() || refs.info.lock().expect(ErrFatal::LOCK).ply > MAX_PLY {
            return evaluate(&refs.board.lock().expect(ErrFatal::LOCK));
        }

        let is_check;
        {
            let board = refs.board.lock().expect(ErrFatal::LOCK);
            is_check = refs.mg.square_attacked(
                &board,
                board.king_square(board.state.active_color),
                board.state.active_color ^ 1,
            );
        }

        if is_check {
            // if we're in check we can't start quiescence search, so we need to increase depth
            depth += 1;
        }
        if depth == 0 {
            return Self::quiescence_search(refs, alpha, beta, pv);
        }
        refs.info.lock().expect(ErrFatal::LOCK).nodes += 1;

        let is_root = refs.info.lock().expect(ErrFatal::LOCK).ply == 0;
        // only try to load from the tt if it's not the first move
        let mut tt_move = None;
        if !is_root {
            let mut tt_eval = None;
            // try to get value from the transposition table
            if let Some(data) = refs
                .tt
                .lock()
                .expect(ErrFatal::LOCK)
                .get(refs.board.lock().expect(ErrFatal::LOCK).state.zobrist_hash)
            {
                let (eval, m) = data.get_values(alpha, beta, depth);
                tt_eval = eval;
                tt_move = Some(m);
            }
            if let Some(eval) = tt_eval {
                return eval;
            }
        }

        let mut legal_moves = 0;
        let mut eval_type = EvalType::Alpha;

        let mut best_eval = -Eval::INF;
        let mut best_move = Move::default();

        let mut moves = refs
            .mg
            .get_all_legal_moves(&refs.board.lock().expect(ErrFatal::LOCK), false);
        moves.reorder(tt_move);

        for &m in moves.iter() {
            let legal = refs
                .board
                .lock()
                .expect(ErrFatal::LOCK)
                .make_move(m, &refs.mg);
            if !legal {
                continue;
            }

            legal_moves += 1;
            {
                let mut info = refs.info.lock().expect(ErrFatal::LOCK);
                info.ply += 1;
                if info.ply >= info.seldepth {
                    info.seldepth = info.ply;
                }
            }
            let mut node_pv = Vec::new();

            let mut eval = 0;
            if !Self::is_draw(&refs.board.lock().expect(ErrFatal::LOCK)) {
                if is_root {
                    thread::scope(|s| {
                        s.spawn(|| {
                            eval = -Self::alpha_beta(depth - 1, -beta, -alpha, &mut node_pv, refs);
                        });
                    });
                } else {
                    eval = -Self::alpha_beta(depth - 1, -beta, -alpha, &mut node_pv, refs);
                }
            }

            refs.board.lock().expect(ErrFatal::LOCK).unmake();
            refs.info.lock().expect(ErrFatal::LOCK).ply -= 1;

            if eval > best_eval {
                best_eval = eval;
                best_move = m;
            }

            // the move is too good for the opponent, stop searching
            if eval >= beta {
                refs.tt.lock().expect(ErrFatal::LOCK).insert(SearchData {
                    depth,
                    eval: beta,
                    eval_type: EvalType::Beta,
                    zobrist_hash: refs.board.lock().expect(ErrFatal::LOCK).state.zobrist_hash,
                    best_move,
                });
                return beta;
            }

            // the move is great for us
            if eval > alpha {
                eval_type = EvalType::Exact;
                alpha = eval;

                pv.clear();
                pv.push(m);
                pv.append(&mut node_pv);
            }
        }

        // finished the loop if there are no legal moves it's either mate or a draw
        if legal_moves == 0 {
            if is_check {
                return -Eval::CHECKMATE + refs.info.lock().expect(ErrFatal::LOCK).ply as i16;
            } else {
                return Eval::STALEMATE; // draw
            }
        }

        refs.tt.lock().expect(ErrFatal::LOCK).insert(SearchData {
            depth,
            eval: alpha,
            eval_type,
            zobrist_hash: refs.board.lock().expect(ErrFatal::LOCK).state.zobrist_hash,
            best_move,
        });

        // didn't beat alpha
        alpha
    }

    pub fn check_termination(refs: &SearchRefs) {
        use crate::search::defs::SearchTime::*;

        if let Ok(data) = refs.control_rx.try_recv() {
            match data {
                SearchControl::Stop => {
                    *refs.terminate.lock().expect(ErrFatal::LOCK) = SearchTerminate::Stop;
                    return;
                }
                SearchControl::Quit => {
                    *refs.terminate.lock().expect(ErrFatal::LOCK) = SearchTerminate::Quit;
                    return;
                }
                _ => (),
            }
        }

        let elapsed = refs.timer_elapsed();
        let stop = match refs.time_control {
            Adaptive(_) => elapsed >= refs.info.lock().expect(ErrFatal::LOCK).allocated_time,
            Depth(d) => refs.info.lock().expect(ErrFatal::LOCK).depth > d,
            Nodes(n) => refs.info.lock().expect(ErrFatal::LOCK).nodes > n,
            MoveTime(t) => elapsed > t,
            Infinite => false,
        };
        if stop {
            *refs.terminate.lock().expect(ErrFatal::LOCK) = SearchTerminate::Stop;
        }
    }
}
