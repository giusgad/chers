use crate::consts::{Piece, Square, MASK_3, MASK_6};

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

impl From<u32> for Move {
    fn from(value: u32) -> Self {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct Move {
    pub data: u32,
}

pub struct MoveOffsets;
impl MoveOffsets {
    pub const FROM: usize = 3;
    pub const TO: usize = 9;
    pub const TYPE: usize = 15;
    pub const TARGET: usize = 18;
}

impl Move {
    pub fn new(piece: Piece, from: Square, to: Square, move_type: MoveType, target: Piece) -> Self {
        let move_type: usize = move_type.into();
        let data = (piece
            | from << MoveOffsets::FROM
            | to << MoveOffsets::TO
            | move_type << MoveOffsets::TYPE
            | target << MoveOffsets::TARGET) as u32;
        Move { data }
    }

    pub fn piece(&self) -> Piece {
        (self.data & MASK_3) as Piece
    }
    pub fn from(&self) -> Square {
        ((self.data >> MoveOffsets::FROM) & MASK_6) as Square
    }
    pub fn to(&self) -> Square {
        ((self.data >> MoveOffsets::TO) & MASK_6) as Square
    }
    pub fn move_type(&self) -> MoveType {
        // TODO: maybe handle the error
        ((self.data >> MoveOffsets::TYPE) & MASK_3)
            .try_into()
            .unwrap()
    }
    pub fn target_piece(&self) -> Piece {
        ((self.data >> MoveOffsets::TARGET) & MASK_3) as Piece
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
impl Into<usize> for MoveType {
    fn into(self) -> usize {
        use MoveType::*;
        match self {
            Quiet => 0,
            Capture => 1,
            DoubleStep => 2,
            Castling => 3,
            Promotion => 4,
            EnPassant => 5,
        }
    }
}

#[derive(Debug)]
pub enum MoveDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl MoveDirection {
    pub const VALUES: [MoveDirection; 8] = [
        MoveDirection::N,
        MoveDirection::NE,
        MoveDirection::E,
        MoveDirection::SE,
        MoveDirection::S,
        MoveDirection::SW,
        MoveDirection::W,
        MoveDirection::NW,
    ];
    pub const fn bb_val(&self) -> i8 {
        match self {
            MoveDirection::N => 8,
            MoveDirection::NE => 9,
            MoveDirection::E => 1,
            MoveDirection::SE => -7,
            MoveDirection::S => -8,
            MoveDirection::SW => -9,
            MoveDirection::W => -1,
            MoveDirection::NW => 7,
        }
    }
}
