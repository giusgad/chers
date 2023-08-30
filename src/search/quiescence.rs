use crate::{eval::evaluate, moves::defs::Move};

use super::{
    defs::{SearchRefs, MAX_PLY},
    Search,
};

impl Search {
    pub fn quiescence_search(
        refs: &mut SearchRefs,
        mut alpha: i16,
        beta: i16,
        pv: &mut Vec<Move>,
    ) -> i16 {
        refs.info.nodes += 1;

        // standing pat
        let stand_pat = evaluate(&refs.board);
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        Self::check_termination(refs);
        if refs.info.ply >= MAX_PLY || refs.stopped() {
            return stand_pat;
        }

        let moves = refs.mg.get_all_legal_moves(&refs.board, true);

        for m in moves.iter() {
            let legal = refs.board.make_move(*m, &refs.mg);
            if !legal {
                continue;
            }

            refs.info.ply += 1;
            if refs.info.ply >= refs.info.seldepth {
                refs.info.seldepth = refs.info.ply;
            }

            let mut node_pv = Vec::new();

            let eval = -Self::quiescence_search(refs, -beta, -alpha, &mut node_pv);

            refs.board.unmake();
            refs.info.ply -= 1;

            if eval >= beta {
                return beta;
            }
            if eval > alpha {
                alpha = eval;
                pv.clear();
                pv.push(*m);
                pv.append(&mut node_pv);
            }
        }

        alpha
    }
}
