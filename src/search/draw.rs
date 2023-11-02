use super::Search;
use crate::{
    board::{defs::Pieces, Board},
    defs::Colors,
};

impl Search {
    pub fn is_draw(board: &Board) -> bool {
        let fifty_move = board.state.halfmove_count >= 50;

        fifty_move || Self::is_threefold(board) || Self::is_material_draw(board)
    }

    fn is_threefold(board: &Board) -> bool {
        let mut count = 1;
        let current = board.state.zobrist_hash;
        for state in board.history.iter() {
            // consider it a draw when there is more than one repetition so that the engine doesn't
            // repeat moves pointlessly
            if count > 1 {
                return true;
            }
            if state.zobrist_hash == current {
                count += 1;
            }
        }
        false
    }

    fn is_material_draw(board: &Board) -> bool {
        let only_kings = board.state.material.iter().sum::<u16>() == 0;
        if only_kings {
            return true;
        }
        let pieces = board.piece_bbs;

        // it's not a draw if there is at least a queen, rook or pawn
        let pawns =
            pieces[Colors::WHITE][Pieces::PAWN] > 0 || pieces[Colors::BLACK][Pieces::PAWN] > 0;
        let queens =
            pieces[Colors::WHITE][Pieces::QUEEN] > 0 || pieces[Colors::BLACK][Pieces::QUEEN] > 0;
        let rooks =
            pieces[Colors::WHITE][Pieces::ROOK] > 0 || pieces[Colors::BLACK][Pieces::ROOK] > 0;
        if pawns || queens || rooks {
            // there are still pawns or queens
            return false;
        }

        // neither side has pawns, queens or rooks so at least one needs two minor pieces
        let w_bishop_knight =
            pieces[Colors::WHITE][Pieces::BISHOP] > 0 && pieces[Colors::WHITE][Pieces::KNIGHT] > 0;
        let b_bishop_knight =
            pieces[Colors::BLACK][Pieces::BISHOP] > 0 && pieces[Colors::BLACK][Pieces::KNIGHT] > 0;
        // if neither side has at least two minor pieces it's a draw
        !(w_bishop_knight || b_bishop_knight)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::defs::START_FEN;

    #[test]
    fn is_draw() {
        let mut b = Board::new();
        let draws = &[
            "5Qqk/8/6p1/2p1r3/pp2P3/1P4P1/P5B1/6K1 w - - 61 77",
            "8/8/8/8/3k4/8/3K4/8 w - - 0 1",
            "8/2k1b3/8/8/8/8/5N2/2K5 w - - 0 1",
        ];
        let not_draws = &[
            START_FEN,
            "8/2k1b3/8/8/8/8/5N2/2K1R3 w - - 0 1",
            "8/8/8/8/2k5/Q3K3/8/8 w - - 0 1",
        ];

        for fen in draws {
            b.read_fen(fen).unwrap();
            assert!(Search::is_draw(&b));
        }
        for fen in not_draws {
            b.read_fen(fen).unwrap();
            assert!(!Search::is_draw(&b));
        }
    }
}
