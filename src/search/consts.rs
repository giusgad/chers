use crate::consts::{Piece, Square};

// A move is represented as a struct to be able to attach decoding functions
// Move.data contains all the information for the move encoded in bits as follows (from LSB):
// | DATA              | BITS   | TYPE
// -------------------------------------
// | piece that moved  | 3 bits | Piece
// | from              | 6 bits | Square
// | to                | 6 bits | Square
// | move type         | 3 bits | MoveType
// | target piece      | 3 bits | Piece
//
// 21 bits total
// target piece is the captured piece in case of a capture or promoted_to in case of promotion
//
// representation:
// 000 000 000000 000000 000

const MASK_3: u32 = 0b111;
const MASK_6: u32 = 0b111111;

impl From<u32> for Move {
    fn from(value: u32) -> Self {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct Move {
    pub data: u32,
}

impl Move {
    pub fn piece(&self) -> Piece {
        let res = self.data & MASK_3;
        res as Piece
    }
    pub fn from(&self) -> Square {
        let res = (self.data >> 3) & MASK_6;
        res as Square
    }
    pub fn to(&self) -> Square {
        let res = (self.data >> 9) & MASK_6;
        res as Square
    }
    pub fn move_type(&self) -> MoveType {
        let res = (self.data >> 15) & MASK_3;
        res.try_into().unwrap() // TODO: maybe handle the error
    }
    pub fn target_piece(&self) -> Piece {
        let res = (self.data >> 18) & MASK_3;
        res as Piece
    }
}

#[derive(Debug)]
pub enum MoveType {
    Quiet,
    Capture,
    DoubleStep,
    Castling,
    Promotion,
    EnPassant,
}

impl TryFrom<u32> for MoveType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use MoveType::*;
        match value {
            0 => Ok(Quiet),
            1 => Ok(Capture),
            2 => Ok(DoubleStep),
            3 => Ok(Castling),
            4 => Ok(Promotion),
            5 => Ok(EnPassant),
            _ => Err("Not a valid movetype"),
        }
    }
}
