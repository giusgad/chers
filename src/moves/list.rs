use super::defs::{Move, MoveType};
use crate::defs::{MAX_LEGAL_MOVES, PIECE_VALUES};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ExtMove {
    pub m: Move,
    s: u16,
}
impl PartialOrd for ExtMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.s.partial_cmp(&other.s)
    }
}

pub struct MoveList {
    moves: [ExtMove; MAX_LEGAL_MOVES],
    len: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [ExtMove::default(); MAX_LEGAL_MOVES],
            len: 0,
        }
    }
}

impl MoveList {
    pub fn push(&mut self, m: Move) {
        self.moves[self.len] = ExtMove { m, s: 0 };
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

struct MoveOrdering;
impl MoveOrdering {
    const TT: u16 = u16::MAX;
    const PROMOTION: u16 = 15000;
    const CAPTURE: u16 = 10000;
    const KILLER: u16 = 9000;
    const CASTLING: u16 = 5000;
}
/// implement the logic for giving scores to the moves according to an heuristic
/// and for selecting the nth best move
impl MoveList {
    /// perform a partial selection sort up to the specified limit, only ordering the first n moves
    /// and leaving the rest of the list unordered.
    pub fn partial_selection_sort(&mut self, limit: usize) {
        for i in 0..limit.min(self.len) {
            let mut max = self.moves[i].s;
            let mut max_i = i;
            for j in i..self.len {
                if self.moves[j].s > max {
                    max = self.moves[j].s;
                    max_i = j;
                }
            }
            self.moves.swap(max_i, i);
        }
    }

    pub fn nth(&mut self, i: usize) -> Move {
        self.partial_selection_sort(i + 1);
        // print!("{} -> ", self.moves[i].s);
        self.moves[i].m
    }

    pub fn give_scores(&mut self, tt_move: Option<Move>, killer_moves: Option<&[Move; 2]>) {
        for i in 0..self.len {
            let curr = &mut self.moves[i];
            // the current move is the tt_move so it gets scored as best
            if tt_move.is_some_and(|m| curr.m == m) {
                curr.s = MoveOrdering::TT;
                continue;
            }
            if let Some(killers) = killer_moves {
                let killer_index = killers.iter().position(|&m| m == curr.m);
                if let Some(i) = killer_index {
                    curr.s = MoveOrdering::KILLER - (i as u16);
                    continue;
                }
            }

            if curr.m.move_type() == MoveType::Capture {
                curr.s += MoveOrdering::CAPTURE;
                // order by Most Valuable Victim (higher score for better victim)
                curr.s += PIECE_VALUES[curr.m.captured_piece()];
            }
            if curr.m.is_castling() {
                curr.s += MoveOrdering::CASTLING;
            }
            if curr.m.is_promotion() {
                curr.s += MoveOrdering::PROMOTION;
            }
        }
    }
}

impl std::ops::Deref for MoveList {
    type Target = [ExtMove];

    fn deref(&self) -> &Self::Target {
        &self.moves[..self.len]
    }
}
