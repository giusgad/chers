use super::Search;
use crate::{
    board::Board,
    eval::evaluate,
    moves::{consts::Move, MoveGenerator},
};
use std::sync::Arc;

impl Search {
    pub fn alpha_beta(
        board: &mut Board,
        mg: &Arc<MoveGenerator>,
        depth: u8,
        mut alpha: i16,
        beta: i16,
    ) -> i16 {
        if depth == 0 {
            return evaluate(&board);
        }

        let mut legal_moves = 0;
        let mut best_move = Move { data: 0 };
        let mut best_eval = i16::MIN;

        let moves = mg.get_all_legal_moves(&board);

        for m in moves.iter() {
            let legal = board.make_move(*m, mg);
            if format!("{m}") == "d6e5" {
                println!("eval:{legal}")
            }
            if !legal {
                continue;
            }
            legal_moves += 1;

            let eval = -Self::alpha_beta(board, mg, depth - 1, -beta, -alpha);

            board.unmake();

            if eval > best_eval {
                best_eval = eval;
                best_move = *m;
            }

            if eval > beta {
                return beta;
            }

            if eval > alpha {
                alpha = eval;
            }
        }

        // finished the loop if there are no legal moves it's either mate or a draw
        if legal_moves == 0 {
            let color = board.state.active_color;
            if mg.square_attacked(&board, board.king_square(color), color ^ 1) {
                return i16::MAX; // TODO: for nicer mate representation use mate_eval-ply to indicate in
                                 // how many moves the mate occurs. https://www.reddit.com/r/chess/comments/ioldx9/why_do_chess_engines_evaluate_checkmate_at_3180/
            } else {
                return 0; // draw
            }
        }

        println!(
            "legal moves found: {legal_moves}, best_move: {best_move}, best_eval: {best_eval}"
        );

        alpha
    }
}
