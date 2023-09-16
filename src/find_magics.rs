use crate::{defs::Piece, moves::MoveGenerator};

const BISHOP_TABLE_SIZE: usize = 5248;
const ROOK_TABLE_SIZE: usize = 102400;

fn find_magics(piece: Piece) {
    let mg = MoveGenerator::new();
}
