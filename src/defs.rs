use crate::{search::defs::SearchResult, uci::defs::UciData};

pub type Bitboard = u64;
pub type Piece = usize;
pub type Color = usize;
pub type Square = usize;

pub const MASK_3: u32 = 0b111;
pub const MASK_6: u32 = 0b111111;
pub const MASK_8: u64 = 255;

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
    pub const FILES: usize = 8;
    pub const SQUARES: usize = 64;
}

pub const MAX_MOVE_COUNT: usize = 1024;
pub const MAX_LEGAL_MOVES: u8 = u8::MAX; // 255

pub const PIECE_VALUES: [u16; NrOf::PIECE_TYPES] = [0, 900, 500, 330, 320, 100];

pub enum Info {
    Search(SearchResult),
    Uci(UciData),
}

pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct ErrFatal;
impl ErrFatal {
    pub const THREAD_JOIN: &'static str = "Error joining thread";
    pub const STDIN: &'static str = "Error reading from stdin";
    pub const LOCK: &'static str = "Error locking mutex";
    pub const RX_RECV: &'static str = "Error with channel recv";
    pub const TX_SEND: &'static str = "Error with channel send";
}
