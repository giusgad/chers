pub type Bitboard = u64;
pub type Piece = usize;
pub type Color = usize;
pub type Square = usize;

pub struct Colors;
impl Colors {
    pub const WHITE: Color = 0;
    pub const BLACK: Color = 1;
    pub const BOTH: Color = 2;
}

pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const RANKS: usize = 8;
    pub const SQUARES: usize = 64;
}
