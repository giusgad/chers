use crate::{
    board::consts::{Files, Pieces, FILE_BBS, SQUARE_BBS},
    consts::{Piece, Square, MASK_3, MASK_6},
    utils::remove_from_vec,
};

// A move is represented as a struct to be able to attach decoding functions
// Move.data contains all the information for the move encoded in bits as follows (from LSB):
// | DATA              | BITS   | TYPE
// -------------------------------------
// | piece that moved  | 3 bits | Piece
// | from              | 6 bits | Square
// | to                | 6 bits | Square
// | move type         | 3 bits | MoveType
// | captured piece    | 3 bits | Piece
// | promoted to       | 3 bits | Piece
// | promotion         | 1 bit  | bool
//
// 25 bits total
// target piece is the captured piece in case of a capture or promoted_to in case of promotion
//
// representation:
// 000 000 000000 000000 000

#[derive(Clone, Copy)]
pub struct Move {
    pub data: u32,
}

pub struct MoveOffsets;
impl MoveOffsets {
    pub const FROM: usize = 3;
    pub const TO: usize = 9;
    pub const TYPE: usize = 15;
    pub const CAPTURED: usize = 18;
    pub const PROMOTED_TO: usize = 21;
    pub const PROMOTION: usize = 24;
}

impl Move {
    pub fn new(
        piece: Piece,
        from: Square,
        to: Square,
        move_type: MoveType,
        captured: Piece,
        promoted_to: Option<Piece>,
    ) -> Self {
        let move_type: usize = move_type.into();
        let mut data = piece
            | from << MoveOffsets::FROM
            | to << MoveOffsets::TO
            | move_type << MoveOffsets::TYPE
            | captured << MoveOffsets::CAPTURED;
        if let Some(prom_piece) = promoted_to {
            data |= 1 << MoveOffsets::PROMOTION | prom_piece << MoveOffsets::PROMOTED_TO;
        } else {
            data |= Pieces::NONE << MoveOffsets::PROMOTED_TO;
        }
        let data = data as u32;
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
    pub fn captured_piece(&self) -> Piece {
        ((self.data >> MoveOffsets::CAPTURED) & MASK_3) as Piece
    }
    pub fn is_promotion(&self) -> bool {
        ((self.data >> MoveOffsets::PROMOTION) & 1) == 1
    }
    pub fn promoted_to(&self) -> Piece {
        ((self.data >> MoveOffsets::PROMOTED_TO) & MASK_3) as Piece
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

#[derive(Debug, PartialEq)]
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

    // return the number that has to be added to the square index
    // to move in the specified direction
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

    // This function returns the possible directions a piece can move in
    // from a starting square, removing horizontal overflow
    pub fn from_pos(square: Square, piece: Piece) -> Vec<Self> {
        let mut res = Vec::from(Self::VALUES);
        let bb_square = SQUARE_BBS[square];

        // handle horizontal overflow
        if bb_square & FILE_BBS[Files::H] > 0 {
            res.drain(1..4);
        } else if bb_square & FILE_BBS[Files::A] > 0 {
            res.drain(5..8);
        }

        // only keep the possible moves for the specified piece
        use MoveDirection::*;
        match piece {
            Pieces::BISHOP => remove_from_vec(&mut res, &[N, E, S, W]),
            Pieces::ROOK => remove_from_vec(&mut res, &[NE, SE, SW, NW]),
            _ => (),
        }
        res
    }
}
