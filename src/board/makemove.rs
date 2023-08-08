use crate::moves::consts::{Move, MoveType};

use super::{consts::Pieces, Board};

impl Board {
    pub fn make_move(&mut self, m: Move) {
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let movetype = m.move_type();
        let en_passant = m.is_en_passant();

        let color = self.state.active_color;

        // remove the piece from its original square
        self.remove_piece(piece, color, from);

        match movetype {
            MoveType::Capture => {
                let captured = m.captured_piece();
                self.put_piece(piece, color, to);
            }
            MoveType::Quiet => {
                self.put_piece(piece, color, to);
            }
            MoveType::Both => panic!("Invalid movetype while making move: {:?}", m),
        }

        match piece {
            // Do all of the following in the above match statement
            Pieces::PAWN => {
                // TODO: handle ep and promotion and doublestep
                let promotion = m.is_promotion();
            }
            Pieces::KING => {
                // remove castling permissions
                let castling = m.is_castling();
            }
            Pieces::ROOK => {
                //TODO: remove castling permissions
            }
            _ => (),
        }

        self.state.active_color ^= 1; // switch active color
    }

    pub fn unmake(&mut self) {}
}
