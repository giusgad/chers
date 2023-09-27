use crate::{
    board::defs::{Files, PieceNames, Pieces, FILE_BBS, SQUARE_BBS, SQUARE_NAMES},
    defs::{Piece, Square, MASK_3, MASK_6},
    utils::remove_from_vec,
};

// A move is represented as a struct to be able to attach decoding functions
// Move.data contains all the information for the move encoded in bits as follows (from LSB):
// | DATA              | BITS   | TYPE
// -------------------------------------
// | piece that moved  | 3 bits | Piece
// | from              | 6 bits | Square
// | to                | 6 bits | Square
// | move type         | 2 bits | MoveType
// | captured piece    | 3 bits | Piece
// | promoted to       | 3 bits | Piece
// | promotion         | 1 bit  | bool
// | en_passant        | 1 bit  | bool
// | castling          | 1 bit  | bool
// | double step       | 1 bit  | bool
//
// 27 bits total
// target piece is the captured piece in case of a capture or promoted_to in case of promotion
//
// representation:
// 000 000 000000 000000 000

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Move {
    pub data: u32,
}

pub struct MoveOffsets;
impl MoveOffsets {
    pub const FROM: usize = 3;
    pub const TO: usize = 9;
    pub const TYPE: usize = 15;
    pub const CAPTURED: usize = 17;
    pub const PROMOTED_TO: usize = 20;
    pub const PROMOTION: usize = 23;
    pub const EN_PASSANT: usize = 24;
    pub const CASTLING: usize = 25;
    pub const DOUBLESTEP: usize = 26;
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
        ((self.data >> MoveOffsets::TYPE) & 0b11)
            .try_into()
            .expect("move type invalid")
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
    pub fn is_en_passant(&self) -> bool {
        ((self.data >> MoveOffsets::EN_PASSANT) & 1) == 1
    }
    pub fn is_castling(&self) -> bool {
        ((self.data >> MoveOffsets::CASTLING) & 1) == 1
    }
    pub fn is_doublestep(&self) -> bool {
        ((self.data >> MoveOffsets::DOUBLESTEP) & 1) == 1
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "piece: {}, from:{}, to:{}, type:{:?}, captured:{}, promotion:{}, promoted to:{}, en_passant:{}, castling:{}, doublestep:{}",
            PieceNames::FULL[self.piece()],
            SQUARE_NAMES[self.from()],
            SQUARE_NAMES[self.to()],
            self.move_type(),
            PieceNames::FULL[self.captured_piece()],
            self.is_promotion(),
            PieceNames::FULL[self.promoted_to()],
            self.is_en_passant(),
            self.is_castling(),
            self.is_doublestep(),
        )
    }
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // algebraic move format
        let prom = if self.is_promotion() {
            PieceNames::CHAR_LOWERCASE[self.promoted_to()].to_string()
        } else {
            "".to_string()
        };
        write!(
            f,
            "{}{}{}",
            SQUARE_NAMES[self.from()],
            SQUARE_NAMES[self.to()],
            prom,
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    Both,
}

impl TryFrom<u32> for MoveType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use MoveType::*;
        match value {
            0 => Ok(Quiet),
            1 => Ok(Capture),
            2 => Ok(Both),
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
            Both => 2,
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
