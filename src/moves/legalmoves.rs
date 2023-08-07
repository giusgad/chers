use crate::{
    board::{consts::Pieces, Board},
    consts::Piece,
    utils::bit_ops,
};

use super::{
    consts::{Move, MoveType},
    MoveGenerator,
};

impl MoveGenerator {
    // This function returns a vec of all the legal moves for a specified piece
    pub fn legal_moves(&self, board: &Board, p: Piece) -> Vec<Move> {
        // TODO: maybe use [Move;64]
        match p {
            Pieces::KING => self.get_king_moves(board),
            _ => Vec::new(),
        }
    }

    fn get_king_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        let color = board.state.active_color;
        let mut king = board.get_pieces(Pieces::KING, color);
        let from = bit_ops::next_one(&mut king);
        let available_bb = self.king[from];

        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];
        let occupied = self_pieces & enemy_pieces;
        let empty = !occupied;

        let mut captures = available_bb & enemy_pieces; // TODO: check if the king is in check after move
        while captures > 0 {
            let to = bit_ops::next_one(&mut captures);
            let m = Move::new(
                Pieces::KING,
                from,
                to,
                MoveType::Capture,
                board.pieces[color ^ 1][to],
                None,
            );
            moves.push(m);
        }
        let mut quiets = empty & available_bb;
        while quiets > 0 {
            let to = bit_ops::next_one(&mut quiets);
            let m = Move::new(
                Pieces::KING,
                from,
                to,
                MoveType::Quiet,
                board.pieces[color ^ 1][to],
                None,
            );
            moves.push(m);
        }

        moves
    }
}
