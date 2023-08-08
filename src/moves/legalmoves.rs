use crate::{
    board::{
        consts::{Castling, Files, Pieces, Ranks, Squares, FILE_BBS, RANK_BBS, SQUARE_BBS},
        Board,
    },
    consts::{Bitboard, Color, Colors, Piece, Square},
    moves::consts::MoveOffsets,
    utils::{add_square_i8, bit_ops},
};

use super::{
    consts::{Move, MoveDirection, MoveType},
    list::MoveList,
    MoveGenerator,
};

impl MoveGenerator {
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

    pub fn castling(&self, board: &Board, list: &mut MoveList) {
        let perms = board.state.castling;
        let color = board.state.active_color;
        let occupied = board.color_bbs[Colors::BLACK] | board.color_bbs[Colors::WHITE];

        let mut piece_bb = board.piece_bbs[color][Pieces::KING];
        let from = bit_ops::next_one(&mut piece_bb);

        let (queenside_perm, kingside_perm): (bool, bool) = match color {
            Colors::WHITE => (perms & Castling::WQ > 0, perms & Castling::WK > 0),
            Colors::BLACK => (perms & Castling::BQ > 0, perms & Castling::BK > 0),
            c => panic!("Invalid active color in castling: {c}"),
        };
        let (to_q, to_k) = match color {
            Colors::WHITE => (Squares::C1, Squares::G1),
            Colors::BLACK => (Squares::C8, Squares::G8),
            c => panic!("Invalid active color in castling: {c}"),
        };

        // can't castle if there are pieces in the way or one of the squares is under attack
        let (mut ok_q, mut ok_k) = (true, true);
        for sq in to_q..from {
            if occupied & SQUARE_BBS[sq] > 0 || Self::square_attacked(&self, board, sq, color ^ 1) {
                ok_q = false;

                break;
            }
        }
        for sq in (from + 1)..=to_k {
            if occupied & SQUARE_BBS[sq] > 0 || Self::square_attacked(&self, board, sq, color ^ 1) {
                ok_k = false;
                break;
            }
        }

        if kingside_perm && ok_k {
            let mut m = Move::new(
                Pieces::KING,
                from,
                to_k,
                MoveType::Quiet,
                Pieces::NONE,
                None,
            );
            m.data |= 1 << MoveOffsets::CASTLING;
            list.push(m);
        }
        if queenside_perm && ok_q {
            let mut m = Move::new(
                Pieces::KING,
                from,
                to_q,
                MoveType::Quiet,
                Pieces::NONE,
                None,
            );
            m.data |= 1 << MoveOffsets::CASTLING;
            list.push(m);
        }
    }

    fn pawn_moves(&self, board: &Board, list: &mut MoveList) {
        // TODO: REDO PAWN MOVES
        // pieces on the 3rd rank don't obstruct pawns from moving to the 4th

        const PROMOTION_PIECES: [Piece; 4] =
            [Pieces::QUEEN, Pieces::BISHOP, Pieces::KNIGHT, Pieces::ROOK];
        let color = board.state.active_color;
        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];
        let empty = !(self_pieces | enemy_pieces);

        let mut pawns_bb = board.get_pieces(Pieces::PAWN, color);
        while pawns_bb > 0 {
            let from = bit_ops::next_one(&mut pawns_bb);
            let dir = if color == Colors::WHITE {
                MoveDirection::N
            } else {
                MoveDirection::S
            };

            if let Some(to) = add_square_i8(from, dir.bb_val()) {
                if SQUARE_BBS[to] & empty > 0 {
                    if (SQUARE_BBS[from] & RANK_BBS[Ranks::R2] > 0 && color == Colors::WHITE)
                        || (SQUARE_BBS[from] & RANK_BBS[Ranks::R7] > 0 && color == Colors::BLACK)
                    {
                        // DOUBLE STEP
                        if let Some(to) = add_square_i8(from, dir.bb_val() * 2) {
                            if SQUARE_BBS[to] & empty > 0 {
                                let mut m = Move::new(
                                    Pieces::PAWN,
                                    from,
                                    to,
                                    MoveType::Quiet,
                                    Pieces::NONE,
                                    None,
                                );
                                m.data |= 1 << MoveOffsets::DOUBLESTEP;
                                list.push(m);
                            }
                        }
                    }
                    if ((SQUARE_BBS[to] & RANK_BBS[Ranks::R8] > 0) && color == Colors::WHITE)
                        || ((SQUARE_BBS[to] & RANK_BBS[Ranks::R1] > 0) && color == Colors::BLACK)
                    {
                        // PROMOTION
                        for promote_to in PROMOTION_PIECES {
                            let m = Move::new(
                                Pieces::PAWN,
                                from,
                                to,
                                MoveType::Quiet,
                                Pieces::NONE,
                                Some(promote_to),
                            );
                            list.push(m);
                        }
                    } else {
                        // QUIET
                        let m =
                            Move::new(Pieces::PAWN, from, to, MoveType::Quiet, Pieces::NONE, None);
                        list.push(m);
                    }
                }
            }

            // EN PASSANT
            let available_bb = self.pawn_capture[color][from];
            let mut captures = available_bb & enemy_pieces;
            if let Some(ep_square) = board.state.ep_square {
                if SQUARE_BBS[ep_square] & available_bb > 0 {
                    let mut m = Move::new(
                        Pieces::PAWN,
                        from,
                        ep_square,
                        MoveType::Capture,
                        Pieces::PAWN,
                        None,
                    );
                    m.data |= 1 << MoveOffsets::EN_PASSANT;
                    list.push(m);
                }
            }

            while captures > 0 {
                let to = bit_ops::next_one(&mut captures);
                if ((SQUARE_BBS[to] & RANK_BBS[Ranks::R8] > 0) && color == Colors::WHITE)
                    || ((SQUARE_BBS[to] & RANK_BBS[Ranks::R1] > 0) && color == Colors::BLACK)
                {
                    for promote_to in PROMOTION_PIECES {
                        let m = Move::new(
                            Pieces::PAWN,
                            from,
                            to,
                            MoveType::Capture,
                            board.pieces[color ^ 1][to],
                            Some(promote_to),
                        );
                        list.push(m);
                    }
                } else {
                    let m = Move::new(
                        Pieces::PAWN,
                        from,
                        to,
                        MoveType::Capture,
                        board.pieces[color ^ 1][to],
                        None,
                    );
                    list.push(m);
                }
            }
        }
    }

    fn non_sliding_moves(&self, piece: Piece, board: &Board, list: &mut MoveList) {
        if piece == Pieces::KING {
            self.castling(board, list);
        }
        let color = board.state.active_color;
        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];
        let empty = !(self_pieces | enemy_pieces);

        let mut piece_bb = board.get_pieces(piece, color);
        while piece_bb > 0 {
            let from = bit_ops::next_one(&mut piece_bb);
            let available_bb = match piece {
                Pieces::KING => self.king[from],
                Pieces::KNIGHT => self.knight[from],
                p => panic!("Invalid piece for non_sliding_moves: {p}"),
            };

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

impl MoveGenerator {
    fn sliding_moves_bb(piece: Piece, board: &Board, sq: Square) -> Bitboard {
        // WARN: the bitboard includes captures on pieces of the same color
        let mut res_bb = 0u64;
        let occupied = board.color_bbs[Colors::WHITE] | board.color_bbs[Colors::BLACK];
        for dir in MoveDirection::from_pos(sq, piece) {
            let mut ray_sq = sq;
            while let Some(i) = add_square_i8(ray_sq, dir.bb_val()) {
                res_bb |= SQUARE_BBS[i];
                if Self::reached_edge(i, &dir) || occupied & SQUARE_BBS[i] > 0 {
                    break;
                }
                ray_sq = i;
            }
        }
        res_bb
    }

    pub fn square_attacked(&self, board: &Board, sq: Square, attacker: Color) -> bool {
        let attacker_pieces = board.piece_bbs[attacker];

        let pawn_bb = self.pawn_capture[attacker ^ 1][sq];
        let king_bb = self.king[sq];
        let knight_bb = self.knight[sq];
        let bishop_bb = Self::sliding_moves_bb(Pieces::BISHOP, board, sq);
        let rook_bb = Self::sliding_moves_bb(Pieces::ROOK, board, sq);
        let queen_bb = bishop_bb | rook_bb;

        // get the attacks starting from the square to check and then, if there are pieces
        // that are on the squares the target square can attack, then it is attacked.
        attacker_pieces[Pieces::PAWN] & pawn_bb > 0
            || attacker_pieces[Pieces::KING] & king_bb > 0
            || attacker_pieces[Pieces::KNIGHT] & knight_bb > 0
            || attacker_pieces[Pieces::BISHOP] & bishop_bb > 0
            || attacker_pieces[Pieces::QUEEN] & queen_bb > 0
            || attacker_pieces[Pieces::ROOK] & rook_bb > 0
    }
}
