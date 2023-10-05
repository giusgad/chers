use crate::{defs::ErrFatal, eval::evaluate, moves::defs::Move};

use super::{
    defs::{SearchRefs, MAX_PLY},
    Search,
};

impl Search {
    pub fn quiescence_search(
        refs: &SearchRefs,
        mut alpha: i16,
        beta: i16,
        pv: &mut Vec<Move>,
    ) -> i16 {
        refs.info.lock().expect(ErrFatal::LOCK).nodes += 1;

        // standing pat
        let stand_pat = evaluate(&refs.board.lock().expect(ErrFatal::LOCK));
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        Self::check_termination(refs);
        if refs.info.lock().expect(ErrFatal::LOCK).ply >= MAX_PLY || refs.stopped() {
            return stand_pat;
        }

        let moves = refs
            .mg
            .get_all_legal_moves(&refs.board.lock().expect(ErrFatal::LOCK), true);

        for m in moves.iter() {
            let legal = refs
                .board
                .lock()
                .expect(ErrFatal::LOCK)
                .make_move(*m, &refs.mg);
            if !legal {
                continue;
            }

            {
                let mut info = refs.info.lock().expect(ErrFatal::LOCK);
                info.ply += 1;
                if info.ply >= info.seldepth {
                    info.seldepth = info.ply;
                }
            }

            let mut node_pv = Vec::new();

            let eval = -Self::quiescence_search(refs, -beta, -alpha, &mut node_pv);

            refs.board.lock().expect(ErrFatal::LOCK).unmake();
            refs.info.lock().expect(ErrFatal::LOCK).ply -= 1;

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
