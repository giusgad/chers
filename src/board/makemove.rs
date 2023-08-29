use crate::{
    defs::{Color, Colors, Piece, Square},
    moves::{
        defs::{Move, MoveType},
        MoveGenerator,
    },
};

use super::{
    defs::{Castling, Pieces, Squares, SQUARE_BBS},
    Board,
};

impl Board {
    pub fn make_move(&mut self, m: Move, mg: &MoveGenerator) -> bool {
        // add the move to the state so that the info can be used when unmaking
        self.state.next_move = m;
        self.history.push(self.state);

        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let movetype = m.move_type();

        let color = self.state.active_color;

        // remove the piece from its original square
        self.remove_piece(piece, color, from);

        // remove the captured piece
        if movetype == MoveType::Capture {
            let captured = m.captured_piece();
            if m.is_en_passant() {
                let ep_capture = match color {
                    Colors::WHITE => to - 8,
                    _ => to + 8,
                };
                self.remove_piece(captured, color ^ 1, ep_capture);
            } else {
                self.remove_piece(captured, color ^ 1, to);
            }
        }

        // put the piece at the destination square
        if m.is_promotion() {
            self.put_piece(m.promoted_to(), color, to)
        } else {
            self.put_piece(piece, color, to);
        }

        // move the rook in castling
        if m.is_castling() {
            self.castle_rook(to);
        }

        // UPDATE STATE
        // set ep_square if there is a pawn doublestep else reset it
        self.clear_ep_square();
        if m.is_doublestep() {
            let sq = match color {
                Colors::WHITE => from + 8,
                _ => from - 8,
            };
            self.set_ep_square(sq)
        }

        // increment move count if black moved
        if color == Colors::BLACK {
            self.state.fullmove_count += 1;
        }

        // halfmove count for 50 move rule
        if movetype == MoveType::Capture || piece == Pieces::PAWN {
            self.state.halfmove_count = 0;
        } else {
            self.state.halfmove_count += 1;
        }

        self.update_castling_perms(piece, color, from);

        self.state.active_color ^= 1; // switch active color
        self.state.zobrist_hash ^= self.zobrist.color_hash();

        let is_check = mg.square_attacked(self, self.king_square(color), color ^ 1);
        if is_check {
            self.unmake()
        }

        // uncomment this to check zobrist hashing is working
        // assert_eq!(self.zobrist_from_scratch(), self.state.zobrist_hash);

        !is_check
    }

    fn castle_rook(&mut self, king: Square) {
        let (rook_from, rook_to) = match king {
            Squares::C1 => (Squares::A1, Squares::D1),
            Squares::G1 => (Squares::H1, Squares::F1),
            Squares::C8 => (Squares::A8, Squares::D8),
            Squares::G8 => (Squares::H8, Squares::F8),
            _ => panic!("Invalid castling square when moving rook"),
        };
        let color = self.state.active_color;
        self.remove_piece(Pieces::ROOK, color, rook_from);
        self.put_piece(Pieces::ROOK, color, rook_to);
    }

    fn update_castling_perms(&mut self, piece: Piece, color: Color, from: Square) {
        let before = self.state.castling;
        match piece {
            Pieces::KING => match color {
                Colors::BLACK => {
                    self.state.castling &= !Castling::BQ;
                    self.state.castling &= !Castling::BK;
                }
                _ => {
                    self.state.castling &= !Castling::WQ;
                    self.state.castling &= !Castling::WK;
                }
            },
            Pieces::ROOK => match from {
                Squares::A1 => self.state.castling &= !Castling::WQ,
                Squares::H1 => self.state.castling &= !Castling::WK,
                Squares::A8 => self.state.castling &= !Castling::BQ,
                Squares::H8 => self.state.castling &= !Castling::BK,
                _ => (),
            },
            _ => (),
        }
        self.state.zobrist_hash ^= self.zobrist.castling_hash(before ^ self.state.castling);
    }
}

// piece function without material calculation because it is already calculated when the state is
// reset from history

fn put_piece(b: &mut Board, piece: Piece, color: Color, square: Square) {
    b.piece_bbs[color][piece] |= SQUARE_BBS[square];
    b.color_bbs[color] |= SQUARE_BBS[square];
    b.pieces[color][square] = piece;
}

fn remove_piece(b: &mut Board, piece: Piece, color: Color, square: Square) {
    b.piece_bbs[color][piece] ^= SQUARE_BBS[square];
    b.color_bbs[color] ^= SQUARE_BBS[square];
    b.pieces[color][square] = Pieces::NONE;
}
impl Board {
    pub fn unmake(&mut self) {
        self.state = self.history.pop();
        let m = self.state.next_move;

        let color = self.state.active_color;
        let piece = m.piece();
        let from = m.from();
        let to = m.to();

        if m.is_promotion() {
            remove_piece(self, m.promoted_to(), color, to);
            put_piece(self, Pieces::PAWN, color, from);
        } else {
            remove_piece(self, piece, color, to);
            put_piece(self, piece, color, from);
        }

        if m.is_castling() {
            self.uncastle_rook(to)
        }

        if m.move_type() == MoveType::Capture {
            if m.is_en_passant() {
                let ep_capture = match color {
                    Colors::WHITE => to - 8,
                    _ => to + 8,
                };
                put_piece(self, Pieces::PAWN, color ^ 1, ep_capture)
            } else {
                put_piece(self, m.captured_piece(), color ^ 1, to);
            }
        }

        put_piece(self, piece, color, from);
    }

    fn uncastle_rook(&mut self, king: Square) {
        let (rook_from, rook_to) = match king {
            Squares::C1 => (Squares::A1, Squares::D1),
            Squares::G1 => (Squares::H1, Squares::F1),
            Squares::C8 => (Squares::A8, Squares::D8),
            Squares::G8 => (Squares::H8, Squares::F8),
            _ => panic!("Invalid castling square when moving rook"),
        };
        let color = self.state.active_color;
        remove_piece(self, Pieces::ROOK, color, rook_to);
        put_piece(self, Pieces::ROOK, color, rook_from);
    }
}
