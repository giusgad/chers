use super::Search;
use crate::{
    board::{defs::Pieces, Board},
    defs::{Colors, NrOf},
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
            if count >= 3 {
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

        // count the pieces
        let mut pieces = [[0u8; NrOf::PIECE_TYPES]; Colors::BOTH];
        for (color, color_pieces) in board.pieces.iter().enumerate() {
            for &piece in color_pieces {
                if piece != Pieces::NONE {
                    pieces[color][piece] += 1;
                }
            }
        }

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
