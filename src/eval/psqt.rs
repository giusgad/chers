use crate::defs::NrOf;

type PSQT = [i16; NrOf::SQUARES];

#[rustfmt::skip]
const KING: PSQT = [
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,  10,   0,   5,   0,  10,   0, 
];

#[rustfmt::skip]
const QUEEN: PSQT = [
  -30, -20,  -5,  -5,  -5,  -5, -20, -30,  
  -20, -10,  -5,  -5,  -5,  -5, -10, -20,  
  -20, -10,  10,  20,  20,  10, -10, -20,  
  -20, -10,  10,  10,  10,  10, -10, -20,  
  -20, -10,  10,  10,  10,  10, -10, -20,  
  -20, -10,  -5,  -5,  -5,  -5, -10, -20,  
  -20, -10,  -5,  -5,  -5,  -5, -10, -20,  
  -30, -20, -10,  -5,  -5, -10, -20, -30,  
];

#[rustfmt::skip]
const ROOK: PSQT = [
    5,   5,   5,   5,   5,   5,   5,   5,  
    15, 15,  15,  20,  20,  15,  15,  15,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   0,   0,   0,   0,   0,   0,   0,  
    0,   5,  10,  10,  10,  10,   5,   0,  
];

#[rustfmt::skip]
const BISHOP: PSQT = [
  -20, -10,  -5,  -5,  -5,  -5, -10, -20,  
  -10,   0,   0,   0,   0,   0,   0, -10,  
   -5,   0,   0,   0,   0,   0,   0,  -5,  
   -5,  10,  15,  25,  25,  15,  10,  -5,  
    5,  10,  10,  20,  20,  10,  10,   5,  
    5,   5,  15,  20,  20,  15,   5,   5,  
    5,   5,   5,   5,   5,   5,   5,   5,  
  -20, -10, -10, -10, -10, -10, -10, -20,  
];

#[rustfmt::skip]
const KNIGHT: PSQT = [
  -30, -20, -20, -20, -20, -20, -20, -30,  
  -20, -10, -10, -10, -10, -10, -10, -20,  
  -20,  -5,  15,  15,  15,  15,  -5, -20,  
  -20,  -5,  20,  20,  20,  20,  -5, -20,  
  -20,  -5,  15,  15,  15,  15,  -5, -20,  
  -20, -10,  15,  15,  15,  15, -10, -20,  
  -20, -10, -10, -10, -10, -10, -10, -20,  
  -30,   0, -20, -20, -20, -20,   0, -30,  
];

#[rustfmt::skip]
const PAWN: PSQT = [
    0,   0,   0,   0,   0,   0,   0,   0,  
   60,  60,  60,  60,  70,  60,  60,  60,  
   40,  40,  40,  50,  60,  40,  40,  40,  
   20,  20,  20,  40,  50,  20,  20,  20,  
   10,  10,  15,  30,  40,  15,  10,  10,  
    5,   5,  10,  20,  20,  10,   5,   5,  
    5,   5,   5, -30, -30,   5,   5,   5,  
    0,   0,   0,   0,   0,   0,   0,   0,  
];

pub const PSQTS: [PSQT; NrOf::PIECE_TYPES] = [KING, QUEEN, ROOK, BISHOP, KNIGHT, PAWN];

#[rustfmt::skip]
pub const FLIP: [usize; 64] = [
    56,  57,  58,  59,  60,  61,  62,  63,  
    48,  49,  50,  51,  52,  53,  54,  55,  
    40,  41,  42,  43,  44,  45,  46,  47,  
    32,  33,  34,  35,  36,  37,  38,  39,  
    24,  25,  26,  27,  28,  29,  30,  31,  
    16,  17,  18,  19,  20,  21,  22,  23,  
     8,   9,  10,  11,  12,  13,  14,  15,  
     0,   1,   2,   3,   4,   5,   6,   7,  
];
