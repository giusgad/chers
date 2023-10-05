use crate::{
    board::{
        defs::{Castling, Pieces, Ranks, Squares, RANK_BBS, SQUARE_BBS},
        Board,
    },
    defs::{Bitboard, Color, Colors, Piece, Square},
    moves::defs::MoveOffsets,
    utils::{add_square_i8, bit_ops::BitIterator},
};

use super::{
    defs::{Move, MoveDirection, MoveType},
    list::MoveList,
    magics::{BISHOP_MAGICS, ROOK_MAGICS},
    MoveGenerator,
};

impl MoveGenerator {
    // This function sends to the MoveList all the legal moves for a specified piece
    pub fn piece_legal_moves(
        &self,
        list: &mut MoveList,
        board: &Board,
        p: Piece,
        only_captures: bool,
    ) {
        match p {
            p @ (Pieces::KING | Pieces::KNIGHT) => {
                self.non_sliding_moves(p, board, list, only_captures)
            }
            p @ (Pieces::QUEEN | Pieces::BISHOP | Pieces::ROOK) => {
                self.sliding_moves(p, board, list, only_captures)
            }
            Pieces::PAWN => self.pawn_moves(board, list, only_captures),
            _ => (),
        }
    }

    pub fn castling(&self, board: &Board, list: &mut MoveList) {
        let perms = board.state.castling;
        let color = board.state.active_color;
        let occupied = board.color_bbs[Colors::BLACK] | board.color_bbs[Colors::WHITE];

        let piece_bb = board.piece_bbs[color][Pieces::KING];
        let from = piece_bb.trailing_zeros() as Square;

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

        if (queenside_perm || kingside_perm)
            && ((color == Colors::WHITE && from != Squares::E1)
                || (color == Colors::BLACK && from != Squares::E8))
        {
            panic!("Castling permissions error: king is not on the starting square")
        }

        // can't castle if there are pieces in the way or one of the squares is under attack
        let (mut ok_q, mut ok_k) = (true, true);
        for sq in to_q..=from {
            if self.square_attacked(board, sq, color ^ 1) {
                ok_q = false;
                break;
            }
        }
        for sq in to_q - 1..from {
            if occupied & SQUARE_BBS[sq] > 0 {
                ok_q = false;
                break;
            }
        }

        for sq in from..=to_k {
            if self.square_attacked(board, sq, color ^ 1) {
                ok_k = false;
                break;
            }
        }
        for sq in (from + 1)..=to_k {
            if occupied & SQUARE_BBS[sq] > 0 {
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

    fn pawn_moves(&self, board: &Board, list: &mut MoveList, only_captures: bool) {
        const PROMOTION_PIECES: [Piece; 4] =
            [Pieces::QUEEN, Pieces::BISHOP, Pieces::KNIGHT, Pieces::ROOK];
        let color = board.state.active_color;
        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];
        let empty = !(self_pieces | enemy_pieces);

        let pawns_bb = board.get_piece_bb(Pieces::PAWN, color);
        for from in pawns_bb.bit_iter() {
            let dir = if color == Colors::WHITE {
                MoveDirection::N
            } else {
                MoveDirection::S
            };

            // EN PASSANT
            let available_bb = self.pawn_capture[color][from];
            let captures = available_bb & enemy_pieces;
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

            for to in captures.bit_iter() {
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

            if only_captures {
                continue;
            }

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
        }
    }

    fn non_sliding_moves(
        &self,
        piece: Piece,
        board: &Board,
        list: &mut MoveList,
        only_captures: bool,
    ) {
        if piece == Pieces::KING && !only_captures {
            self.castling(board, list);
        }
        let color = board.state.active_color;
        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];
        let empty = !(self_pieces | enemy_pieces);

        let piece_bb = board.get_piece_bb(piece, color);
        for from in piece_bb.bit_iter() {
            let available_bb = match piece {
                Pieces::KING => self.king[from],
                Pieces::KNIGHT => self.knight[from],
                p => panic!("Invalid piece for non_sliding_moves: {p}"),
            };

            let captures = available_bb & enemy_pieces;
            for to in captures.bit_iter() {
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

            if !only_captures {
                let quiets = available_bb & empty;
                for to in quiets.bit_iter() {
                    let m = Move::new(piece, from, to, MoveType::Quiet, Pieces::NONE, None);
                    list.push(m);
                }
            }
        }
    }

    // this function follows rays in the directions possible for the given piece and adds to the
    // MoveList all of its possible moves
    fn sliding_moves(&self, piece: Piece, board: &Board, list: &mut MoveList, only_captures: bool) {
        let color = board.state.active_color;
        let self_pieces = board.color_bbs[color];
        let enemy_pieces = board.color_bbs[color ^ 1];
        let blocker = self_pieces | enemy_pieces;

        let piece_bb = board.get_piece_bb(piece, color);
        for from in piece_bb.bit_iter() {
            let mut legal_bb = self.get_bb_from_magics(from, blocker, piece);
            legal_bb &= !self_pieces;

            // generate the moves
            for to in legal_bb.bit_iter() {
                let to_bb = SQUARE_BBS[to];

                // it's a capture
                if to_bb & enemy_pieces > 0 {
                    let m = Move::new(
                        piece,
                        from,
                        to,
                        MoveType::Capture,
                        board.pieces[color ^ 1][to],
                        None,
                    );
                    list.push(m);
                } else if !only_captures {
                    // it 's quiet
                    let m = Move::new(piece, from, to, MoveType::Quiet, Pieces::NONE, None);
                    list.push(m);
                }
            }
        }
    }

    fn get_bb_from_magics(&self, sq: Square, blocker: Bitboard, piece: Piece) -> Bitboard {
        match piece {
            Pieces::ROOK => {
                let rook_idx = ROOK_MAGICS[sq]
                    .get_index(Self::simplify_blocker(blocker & self.rook_masks[sq], sq));
                self.rook[rook_idx]
            }
            Pieces::BISHOP => {
                let bishop_idx = BISHOP_MAGICS[sq]
                    .get_index(Self::simplify_blocker(blocker & self.bishop_masks[sq], sq));
                self.bishop[bishop_idx]
            }
            Pieces::QUEEN => {
                let bishop_idx = BISHOP_MAGICS[sq]
                    .get_index(Self::simplify_blocker(blocker & self.bishop_masks[sq], sq));
                let bishop = self.bishop[bishop_idx];
                let rook_idx = ROOK_MAGICS[sq]
                    .get_index(Self::simplify_blocker(blocker & self.rook_masks[sq], sq));
                let rook = self.rook[rook_idx];
                bishop | rook
            }
            _ => panic!("Invalid piece"),
        }
    }
}

impl MoveGenerator {
    pub fn square_attacked(&self, board: &Board, sq: Square, attacker: Color) -> bool {
        let attacker_pieces = board.piece_bbs[attacker];
        let occupied = board.color_bbs[Colors::WHITE] | board.color_bbs[Colors::BLACK];

        let pawn_bb = self.pawn_capture[attacker ^ 1][sq];
        let king_bb = self.king[sq];
        let knight_bb = self.knight[sq];
        let bishop_bb = self.get_bb_from_magics(sq, occupied, Pieces::BISHOP);
        let rook_bb = self.get_bb_from_magics(sq, occupied, Pieces::ROOK);
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
