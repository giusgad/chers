use super::defs::{Move, MoveType};
use crate::defs::MAX_LEGAL_MOVES;

pub struct MoveList {
    pub list: [Move; MAX_LEGAL_MOVES],
    pub index: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            list: [Move::default(); MAX_LEGAL_MOVES],
            index: 0,
        }
    }
}

impl MoveList {
    pub fn push(&mut self, m: Move) {
        self.list[self.index] = m;
        self.index += 1;
    }

    fn swap(&mut self, i: usize, j: usize) {
        (self.list[i], self.list[j]) = (self.list[j], self.list[i]);
    }

    // reorder the moves for a faster search
    pub fn reorder(&mut self, tt_move: Option<Move>) {
        // if a move retrieved from the tt is available it will be repositioned to the top
        // so we leave a space for it at the beginning of the list
        let mut top = if tt_move.is_some() { 1 } else { 0 };

        #[allow(clippy::mut_range_bound)]
        for i in top..self.index {
            let m = &self.list[i];
            if tt_move.is_some_and(|tt_m| &tt_m == m) {
                self.swap(0, i);
            } else if m.move_type() == MoveType::Capture {
                self.swap(top, i);
                top += 1;
            }
        }
    }
}

impl std::ops::Deref for MoveList {
    type Target = [Move];

    fn deref(&self) -> &Self::Target {
        &self.list[..self.index]
    }
}
