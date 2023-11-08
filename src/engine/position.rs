use crate::{
    board::{
        defs::{square_by_name, PieceNames, Pieces, SQUARE_NAMES},
        Board,
    },
    defs::{ErrFatal, Piece, Square, START_FEN},
    moves::{defs::Move, MoveGenerator},
    uci::Uci,
    utils::piece_from_char,
};

use super::Engine;

const ERR_FEN: &str = "Error reading fen, board not changed";
const ERR_MOVE_PARSING: &str = "Error parsing moves, board not changed";

impl Engine {
    pub fn setup_position(&mut self, fen: &str, moves: Vec<String>) {
        // setup fen position
        let fen = if fen.trim() == "startpos" {
            START_FEN
        } else {
            fen
        };
        let Ok(moves) = Self::parse_moves(moves) else {
            Uci::output_err(ERR_MOVE_PARSING);
            return;
        };

        let mut board = self.board.lock().expect(ErrFatal::LOCK);
        let res = board.read_fen(fen);
        if res.is_err() {
            Uci::output_err(ERR_FEN);
            return;
        }

        // play the moves from the gui
        let res = Self::play_moves(&mut board, &self.mg, moves);
        match res {
            Ok(()) => (),
            Err(e) => Uci::output_err(format!("Error Move {e}, is not legal, board changed.")),
        }
    }
}

// smallmove is used to represent a move only by the from and to squares.
// It is used to play a move from a string (by trying all possible moves).
#[derive(Clone, Copy, Debug)]
struct SmallMove {
    from: Square,
    to: Square,
    promotion: Piece,
}
impl TryFrom<String> for SmallMove {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let chars: Vec<char> = s.chars().collect();
        let mut promotion: Piece = Pieces::NONE;
        let from = square_by_name(&chars[0..2].iter().collect::<String>())?;
        let to = square_by_name(&chars[2..4].iter().collect::<String>())?;

        if chars.len() == 5 {
            promotion = piece_from_char(chars[4])?;
        }

        Ok(SmallMove {
            from,
            to,
            promotion,
        })
    }
}
impl Into<String> for SmallMove {
    fn into(self) -> String {
        format!(
            "{}{}{}",
            SQUARE_NAMES[self.from],
            SQUARE_NAMES[self.to],
            PieceNames::CHAR_LOWERCASE[self.promotion]
        )
    }
}
impl PartialEq<Move> for SmallMove {
    fn eq(&self, other: &Move) -> bool {
        self.from == other.from() && self.to == other.to() && self.promotion == other.promoted_to()
    }
}

impl Engine {
    fn parse_moves(strings: Vec<String>) -> Result<Vec<SmallMove>, ()> {
        let mut moves = Vec::with_capacity(strings.len());
        for s in strings {
            match s.try_into() {
                Ok(m) => moves.push(m),
                Err(()) => return Err(()),
            };
        }
        Ok(moves)
    }

    fn play_moves(
        board: &mut Board,
        mg: &MoveGenerator,
        moves: Vec<SmallMove>,
    ) -> Result<(), String> {
        // find a legal move where the from and to squares and the promotion piece are the same of the smallmove's
        for small_move in moves {
            let mut found = false;
            let pseudo_legal = mg.get_all_legal_moves(board, false);
            for pl_move in pseudo_legal.iter().map(|ext| &ext.m) {
                if &small_move == pl_move {
                    found = true;
                    if !board.make_move(*pl_move, mg) {
                        // the move is found in the pseudo legals but can't be executed
                        return Err(small_move.into());
                    }
                }
            }
            if !found {
                return Err(small_move.into());
            }
        }
        Ok(())
    }
}
