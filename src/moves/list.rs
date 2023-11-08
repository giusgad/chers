use super::defs::{Move, MoveType};
use crate::{
    defs::{Color, MAX_LEGAL_MOVES, PIECE_VALUES},
    search::defs::HistoryHeuristic,
};

/// contains a [`Move`] and an u16 that represents the move's score used for move ordering
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

/// contains the constants for scores used in move ordering
struct MoveOrdering;
impl MoveOrdering {
    const TT: u16 = u16::MAX;
    const PROMOTION: u16 = 15000;
    const CAPTURE: u16 = 10000;
    const KILLER: u16 = 9000;
    const CASTLING: u16 = 8000;
}

/// implement the logic for giving scores to the moves according to an heuristic
/// and for selecting the nth best move
impl MoveList {
    /// perform a partial selection sort up to the specified limit, only ordering the first n moves
    /// and leaving the rest of the list unordered.
    fn partial_selection_sort(&mut self, limit: usize) {
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

    /// Returns the move with the score that is nth in the ordered list.
    /// [`MoveList`].`give_scores`() needs to be called first to have the moves ordered.
    pub fn nth(&mut self, i: usize) -> Move {
        self.partial_selection_sort(i + 1);
        self.moves[i].m
    }

    /**
    For all the [`ExtMove`] in the list assign a score related to how
    good the move might be, following the ordering defined in the constants
    in [`MoveOrdering`].

    Uses the following heuristics in order:
    - TT move
    - Promotions
    - Captures ordered by MVV
    - Quiet moves from killer heuristics
    - Castling
    - Quiet moves ordered with history heuristic
    */
    pub fn give_scores(
        &mut self,
        tt_move: Option<Move>,
        killer_moves: Option<&[Move; 2]>,
        history: Option<(&HistoryHeuristic, Color)>,
    ) {
        //TODO: PV first from previous iterative depennig
        //TODO: Static Exchange Evaluation
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
                continue;
            }
            if curr.m.is_promotion() {
                curr.s += MoveOrdering::PROMOTION;
                continue;
            }
            if curr.m.move_type() == MoveType::Quiet {
                if let Some((history, color)) = history {
                    curr.s += history[color][curr.m.from()][curr.m.to()];
                    if curr.s >= MoveOrdering::CASTLING {
                        curr.s = MoveOrdering::CASTLING - 1;
                    }
                }
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
