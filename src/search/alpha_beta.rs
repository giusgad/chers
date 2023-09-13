use super::{
    defs::{SearchRefs, MAX_PLY},
    Search,
};
use crate::{
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
        refs: &mut SearchRefs,
    ) -> i16 {
        // check if the search has to stop
        Self::check_termination(refs);
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
        if depth <= 0 {
            return Self::quiescence_search(refs, alpha, beta, pv);
        }

        refs.info.nodes += 1;

        let mut legal_moves = 0;
        let mut best_eval = -Eval::INF;

        let mut moves = refs.mg.get_all_legal_moves(refs.board, false);
        moves.reorder();

        for m in moves.iter() {
            let legal = refs.board.make_move(*m, refs.mg);
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
            if !Self::is_draw(&refs.board) {
                eval = -Self::alpha_beta(depth - 1, -beta, -alpha, &mut node_pv, refs);
            }

            refs.board.unmake();
            refs.info.ply -= 1;

            if eval > best_eval {
                best_eval = eval;
                pv.clear();
                pv.push(*m);
                pv.append(&mut node_pv);
            }

            if eval >= beta {
                return beta;
            }

            if eval > alpha {
                alpha = eval;
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

        /* let moves: String = moves.iter().fold(String::new(), |mut s, m| {
            s.push_str(&format!("{}", m));
            s.push(' ');
            s
        });
        println!(
            "legal moves found: {legal_moves}, pv: {}, best_eval: {best_eval}",
            moves
        ); */

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
