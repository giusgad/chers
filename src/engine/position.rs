use crate::{
    board::{
        defs::{square_by_name, Pieces},
        Board,
    },
    defs::{ErrFatal, Piece, Square},
    moves::{defs::Move, MoveGenerator},
    utils::piece_from_char,
};

use super::Engine;

const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Engine {
    pub fn setup_position(&mut self, fen: String, moves: Vec<String>) {
        // setup fen position
        let fen = if fen.trim() == "startpos" {
            START_FEN
        } else {
            fen.as_str()
        };
        let moves = Self::parse_moves(moves);

        // TODO: maybe don't panic here (remove .expect)
        let mut board = self.board.lock().expect(ErrFatal::LOCK);
        board.read_fen(fen).unwrap();

        // play the moves from the gui
        Self::play_moves(&mut board, &self.mg, moves).unwrap();
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
impl PartialEq<Move> for SmallMove {
    fn eq(&self, other: &Move) -> bool {
        self.from == other.from() && self.to == other.to() && self.promotion == other.promoted_to()
    }
}

impl Engine {
    fn parse_moves(strings: Vec<String>) -> Vec<SmallMove> {
        let mut moves = Vec::with_capacity(strings.len());
        for s in strings {
            match s.try_into() {
                Ok(m) => moves.push(m),
                Err(_) => continue,
            };
        }
        moves
    }

    fn play_moves(board: &mut Board, mg: &MoveGenerator, moves: Vec<SmallMove>) -> Result<(), ()> {
        // find a legal move where the from and to squares and the promotion piece are the same of the smallmove's
        for small_move in moves {
            let pseudo_legal = mg.get_all_legal_moves(&board);
            for pl_move in pseudo_legal.iter() {
                if &small_move == pl_move {
                    if !board.make_move(*pl_move, mg) {
                        return Err(());
                    }
                }
            }
        }
        Ok(())
    }
}
