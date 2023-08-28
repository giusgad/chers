use super::{defs::SearchRefs, Search};
use crate::{
    board::defs::Pieces,
    defs::PIECE_VALUES,
    eval::{defs::Eval, evaluate},
    moves::defs::Move,
    search::defs::{SearchControl, SearchTerminate},
};

impl Search {
    pub fn alpha_beta(
        depth: u8,
        mut alpha: i16,
        beta: i16,
        pv: &mut Vec<Move>,
        refs: &mut SearchRefs,
    ) -> i16 {
        // check if the search has to stop
        Self::check_termination(refs);
        if depth == 0 || refs.stopped() {
            return evaluate(refs.board);
        }
        let mut legal_moves = 0;
        let mut best_eval = -Eval::INF;

        let moves = refs.mg.get_all_legal_moves(refs.board);

        for m in moves.iter() {
            let legal = refs.board.make_move(*m, refs.mg);
            if !legal {
                continue;
            }

            legal_moves += 1;
            refs.info.ply += 1;

            let mut node_pv = Vec::new();

            let eval = -Self::alpha_beta(depth - 1, -beta, -alpha, &mut node_pv, refs);

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
            let color = refs.board.state.active_color;
            if refs
                .mg
                .square_attacked(refs.board, refs.board.king_square(color), color ^ 1)
            {
                return -Eval::CHECKMATE
                    + refs.info.ply as i16 * PIECE_VALUES[Pieces::QUEEN] as i16;
            // TODO: for nicer mate representation use mate_eval-ply to indicate in
            // how many moves the mate occurs. https://www.reddit.com/r/chess/comments/ioldx9/why_do_chess_engines_evaluate_checkmate_at_3180/
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

    fn check_termination(refs: &mut SearchRefs) {
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
