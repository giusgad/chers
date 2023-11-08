use crate::{
    defs::{Colors, NrOf},
    utils::piece_from_char,
};

use super::{
    defs::{square_by_name, Castling},
    Board,
};

// fen notation is composed by: piecepositions activecolor castling enpassant halfmovecount fullmovecount

#[derive(Debug)]
pub enum FenError {
    Length,
    InvalidPiece,
    ActiveColor,
    Castling,
    EpSquare,
    HalfMove,
    FullMove,
}

impl Board {
    pub fn read_fen(&mut self, fen: &str) -> Result<(), FenError> {
        let fen_split: Vec<&str> = fen.trim().split(' ').collect();
        if fen_split.len() != 6 {
            return Err(FenError::Length);
        }

        let mut fen_iter = fen_split.iter();

        let mut board = Board::new();

        // READ PIECE POSITIONS
        let mut rank = NrOf::RANKS - 1;
        let mut file = 0usize;
        for c in fen_iter.next().unwrap().chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                c if c.is_ascii_digit() => {
                    let n = (c.to_digit(10).unwrap() as usize) + file;
                    file = match n {
                        n @ 0..=7 => n,
                        n => n % NrOf::FILES,
                    };
                }
                c => {
                    if let Ok(piece) = piece_from_char(c) {
                        let color = if c.is_uppercase() {
                            Colors::WHITE
                        } else {
                            Colors::BLACK
                        };
                        board.put_piece(piece, color, rank * 8 + file);
                        file += 1;
                    } else {
                        return Err(FenError::InvalidPiece);
                    }
                }
            }
        }

        // ACTIVE COLOR
        for c in fen_iter.next().unwrap().chars() {
            match c {
                'w' => board.state.active_color = Colors::WHITE,
                'b' => {
                    board.state.active_color = Colors::BLACK;
                    board.state.zobrist_hash ^= board.zobrist.color_hash();
                }
                _ => return Err(FenError::ActiveColor),
            }
        }

        // CASTLING STATE
        for c in fen_iter.next().unwrap().chars() {
            match c {
                'k' => board.state.castling |= Castling::BK,
                'q' => board.state.castling |= Castling::BQ,
                'K' => board.state.castling |= Castling::WK,
                'Q' => board.state.castling |= Castling::WQ,
                '-' => board.state.castling = 0,
                _ => return Err(FenError::Castling),
            }
        }
        board.state.zobrist_hash ^= board.zobrist.castling_hash(board.state.castling);

        // EN PASSANT SQUARE
        let ep_square = fen_iter.next().unwrap();
        if *ep_square == "-" {
            board.state.ep_square = None;
        } else if let Ok(square) = square_by_name(ep_square) {
            board.set_ep_square(square);
        } else {
            return Err(FenError::EpSquare);
        }

        // HALF MOVES
        let count_str = fen_iter.next().unwrap();
        if let Ok(count) = count_str.parse::<u8>() {
            board.state.halfmove_count = count;
        } else {
            return Err(FenError::HalfMove);
        }

        // FULL MOVES
        let count_str = fen_iter.next().unwrap();
        if let Ok(count) = count_str.parse::<u16>() {
            board.state.fullmove_count = count;
        } else {
            return Err(FenError::FullMove);
        }

        // if everything is ok replace the original board with the new one
        *self = board;
        Ok(())
    }
}
