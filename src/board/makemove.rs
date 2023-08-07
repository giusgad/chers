use crate::moves::consts::Move;

use super::Board;

impl Board {
    pub fn make_move(&mut self, m: Move) {
        //TODO: when making move check if it removes castling permissions
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let movetype = m.move_type();
    }
}
