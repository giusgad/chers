use crate::{
    board::{
        consts::{Files, Pieces, FILE_BBS, SQUARE_BBS},
        Board,
    },
    consts::{Color, Piece, Square},
    utils::{add_square_i8, bit_ops},
};

use super::{
    consts::{Move, MoveDirection, MoveType},
    list::MoveList,
    MoveGenerator,
};

impl MoveGenerator {
    fn square_attacked(&self, board: &Board, sq: Square, attacker: Color) {}

    // This function sends to the MoveList all the legal moves for a specified piece
    pub fn piece_legal_moves(&self, list: &mut MoveList, board: &Board, p: Piece) {
        match p {
            p @ (Pieces::KING | Pieces::KNIGHT) => self.non_sliding_moves(p, board, list),
            p @ (Pieces::QUEEN | Pieces::BISHOP | Pieces::ROOK) => {
                Self::sliding_moves(p, board, list)
            }
            Pieces::PAWN => self.pawn_moves(board, list),
            _ => (),
        }
    }

    fn pawn_moves(&self, board: &Board, list: &MoveList) {}

    fn non_sliding_moves(&self, piece: Piece, board: &Board, list: &mut MoveList) {
        let color = board.state.active_color;
        let mut piece_bb = board.get_pieces(piece, color);
        while piece_bb > 0 {
            let from = bit_ops::next_one(&mut piece_bb);
            let available_bb = match piece {
                Pieces::KING => self.king[from],
                Pieces::KNIGHT => self.knight[from],
                p => panic!("Invalid piece for non_sliding_moves: {p}"),
            };

            let self_pieces = board.color_bbs[color];
            let enemy_pieces = board.color_bbs[color ^ 1];
            let empty = !(self_pieces | enemy_pieces);

            let mut captures = available_bb & enemy_pieces; // TODO: check if the king is in check after move
            while captures > 0 {
                let to = bit_ops::next_one(&mut captures);
                let m = Move::new(
                    piece,
                    from,
                    to,
                    MoveType::Capture,
                    board.pieces[color ^ 1][to],
                    None,
                );
                list.push(m);
            }

            let mut quiets = available_bb & empty;
            while quiets > 0 {
                let to = bit_ops::next_one(&mut quiets);
                let m = Move::new(piece, from, to, MoveType::Quiet, Pieces::NONE, None);
                list.push(m);
            }
        }
    }

    // this function follows rays in the directions possible for the given piece and adds to the
    // MoveList all of its possible moves
    fn sliding_moves(piece: Piece, board: &Board, list: &mut MoveList) {
        let color = board.state.active_color;
        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];

        let mut piece_bb = board.get_pieces(piece, color);
        while piece_bb > 0 {
            let from = bit_ops::next_one(&mut piece_bb);

            for dir in MoveDirection::from_pos(from, piece) {
                let mut ray_sq = from;
                while let Some(to) = add_square_i8(ray_sq, dir.bb_val()) {
                    if ((self_pieces >> to) & 1) == 1 {
                        // The ray reached an ally piece
                        break;
                    } else if (enemy_pieces >> to) & 1 == 1 {
                        // It's a capture
                        let m = Move::new(
                            piece,
                            from,
                            to,
                            MoveType::Capture,
                            board.pieces[color ^ 1][to],
                            None,
                        );
                        list.push(m);
                        break;
                    } else {
                        // It's a quiet move
                        let m = Move::new(piece, from, to, MoveType::Quiet, Pieces::NONE, None);
                        list.push(m);
                        if Self::reached_edge(to, &dir) {
                            // The ray reached the side of the board
                            break;
                        }
                    }
                    ray_sq = to;
                }
            }
        }
    }

    // This function returns true if the ray reached the side of the board in the given direction
    fn reached_edge(sq: Square, dir: &MoveDirection) -> bool {
        use MoveDirection::*;
        let bishop =
            SQUARE_BBS[sq] & FILE_BBS[Files::A] > 0 || SQUARE_BBS[sq] & FILE_BBS[Files::H] > 0;
        let rook = bishop && (dir == &E || dir == &W);
        match dir {
            N | E | S | W => rook,
            _ => bishop,
        }
    }
}
