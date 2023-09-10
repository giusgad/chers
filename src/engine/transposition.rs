use crate::moves::defs::Move;

enum EvalType {
    Exact,
    Alpha,
    Beta,
}

pub struct SearchData {
    best_move: Vec<Move>,
    depth: u8,
    eval: i16,
    eval_type: EvalType,
}

pub struct TT {
    megabytes: usize,
    data: [u64; 10],
}

impl TT {
    pub fn new() -> Self {
        TT {
            megabytes: 0,
            data: [0; 10],
        }
    }
}
