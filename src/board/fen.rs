use crate::consts::NrOf;

use super::Board;

// fen notation is composed by: piecepositions activecolor castling enpassant halfmovecount fullmovecount

pub enum FenError {
    Length,
}

impl Board {
    pub fn fen_read(&mut self, fen: &str) -> Result<(), FenError> {
        let fen_split: Vec<&str> = fen.split(' ').collect();
        if fen_split.len() != 6 {
            return Err(FenError::Length);
        }

        let mut fen_iter = fen_split.iter();

        if let Some(chars) = fen_iter.next() {
            let mut rank = NrOf::RANKS - 1;
            let mut file = 0usize;
            for c in chars.chars() {
                if c.is_uppercase() {
                    // self.put_piece()
                }
            }
        }

        Ok(())
    }
}
