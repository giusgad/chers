use crate::{
    consts::{Color, Colors, Piece, Square},
    moves::consts::{Move, MoveType},
};

use super::{
    consts::{Castling, Pieces, Squares},
    Board,
};

impl Board {
    pub fn make_move(&mut self, m: Move) {
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
            let (rook_from, rook_to) = match to {
                Squares::C1 => (Squares::A1, Squares::D1),
                Squares::G1 => (Squares::H1, Squares::F1),
                Squares::C8 => (Squares::A8, Squares::D8),
                Squares::G8 => (Squares::H8, Squares::F8),
                _ => panic!("Invalid castling square when moving rook"),
            };
            self.remove_piece(Pieces::ROOK, color, rook_from);
            self.put_piece(Pieces::ROOK, color, rook_to);
        }

        // UPDATE STATE
        // set ep_square if there is a pawn doublestep else reset it
        if m.is_doublestep() {
            self.state.ep_square = match color {
                Colors::WHITE => Some(from + 8),
                _ => Some(from - 8),
            };
        } else {
            self.state.ep_square = None;
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
    }

    pub fn unmake(&mut self) {}

    fn update_castling_perms(&mut self, piece: Piece, color: Color, from: Square) {
        match piece {
            Pieces::KING => match color {
                Colors::BLACK => {
                    self.state.castling &= !Castling::BQ;
                    self.state.castling &= !Castling::BK
                }
                _ => {
                    self.state.castling &= !Castling::WQ;
                    self.state.castling &= !Castling::WK
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
    }
}
