use super::{
    defs::{SearchRefs, MAX_PLY},
    Search,
};
use crate::{
    engine::transposition::{EvalType, SearchData},
    eval::{defs::Eval, evaluate},
    moves::defs::{Move, MoveType},
    search::defs::{SearchControl, SearchTerminate},
};

impl Search {
    pub fn alpha_beta(
        mut depth: u8,
        mut alpha: i16,
        beta: i16,
        pv: &mut Vec<Move>,
        refs: &mut SearchRefs,
    ) -> i16 {
        // check if the search has to stop
        // only perform the check every 2048 nodes for efficency,
        // since the calculation of the elapsed time is expensive
        if refs.info.nodes & 2048 == 0 {
            Self::check_termination(refs);
        }
        let is_root = refs.info.ply == 0;

        if refs.stopped() || refs.info.ply > MAX_PLY {
            return evaluate(refs.board);
        }

        let is_check = refs.mg.square_attacked(
            refs.board,
            refs.board.king_square(refs.board.state.active_color),
            refs.board.state.active_color ^ 1,
        );
        if is_check {
            // if we're in check we can't start quiescence search, so we need to increase depth
            depth += 1;
        }
        if depth == 0 {
            return Self::quiescence_search(refs, alpha, beta, pv);
        }

        refs.info.nodes += 1;

        // only try to load from the tt if it's not the first move
        let mut tt_move = None;
        if !is_root {
            let mut tt_eval = None;
            // try to get value from the transposition table
            if let Some(data) = refs.tt.get(refs.board.state.zobrist_hash) {
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
        let ply = refs.info.ply as usize;

        let mut moves = refs.mg.get_all_legal_moves(refs.board, false);
        moves.give_scores(tt_move, Some(&refs.killer_moves[ply]));

        for i in 0..moves.len() {
            let m = moves.nth(i);
            let legal = refs.board.make_move(m, refs.mg);
            if !legal {
                continue;
            }

            legal_moves += 1;
            refs.info.ply += 1;
            if refs.info.ply >= refs.info.seldepth {
                refs.info.seldepth = refs.info.ply;
            }

            let mut node_pv = Vec::new();

            let mut eval = 0;
            // search if it's not a draw OR if we are at the root
            // The is_root check is needed because otherwise the engine will evaluate as a draw
            // any position were a draw by repetition can be reached
            if !Self::is_draw(refs.board) || is_root {
                eval = -Self::alpha_beta(depth - 1, -beta, -alpha, &mut node_pv, refs);
            }

            refs.board.unmake();
            refs.info.ply -= 1;

            if eval > best_eval {
                best_eval = eval;
                best_move = m;
            }

            // the move is too good for the opponent, stop searching
            if eval >= beta {
                refs.tt.insert(SearchData {
                    depth,
                    eval: beta,
                    eval_type: EvalType::Beta,
                    zobrist_hash: refs.board.state.zobrist_hash,
                    best_move,
                });
                if m.move_type() == MoveType::Quiet && refs.killer_moves[ply][0] != m {
                    refs.killer_moves[ply][1] = refs.killer_moves[ply][0];
                    refs.killer_moves[ply][0] = m;
                }
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
                return -Eval::CHECKMATE + refs.info.ply as i16;
            } else {
                return Eval::STALEMATE; // draw
            }
        }

        refs.tt.insert(SearchData {
            depth,
            eval: alpha,
            eval_type,
            zobrist_hash: refs.board.state.zobrist_hash,
            best_move,
        });

        // didn't beat alpha
        alpha
    }

    pub fn check_termination(refs: &mut SearchRefs) {
        use crate::search::defs::SearchTime::*;

        if let Ok(data) = refs.control_rx.try_recv() {
            match data {
                SearchControl::Stop => {
                    refs.terminate = SearchTerminate::Stop;
                    return;
                }
                SearchControl::Quit => {
                    refs.terminate = SearchTerminate::Quit;
                    return;
                }
                _ => (),
            }
        }

        let elapsed = refs.timer_elapsed();
        let stop = match refs.time_control {
            Adaptive(_) => elapsed >= refs.info.allocated_time,
            Depth(d) => refs.info.depth > d,
            Nodes(n) => refs.info.nodes > n,
            MoveTime(t) => elapsed > t,
            Infinite => false,
        };
        if stop {
            refs.terminate = SearchTerminate::Stop;
        }
    }
}
