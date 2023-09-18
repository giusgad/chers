pub struct Eval;
impl Eval {
    pub const INF: i16 = 30_000;
    pub const CHECKMATE: i16 = 29_000;
    pub const CHECKMATE_TRESHOLD: i16 = Eval::CHECKMATE - 1000;
    pub const STALEMATE: i16 = 0;
}
