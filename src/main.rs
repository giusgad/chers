#![allow(dead_code, unused_variables)]

use crate::board::consts::{square_by_name, PieceNames, Pieces, SQUARE_NAMES};
use crate::moves::consts::{Move, MoveType};

mod board;
mod consts;
mod moves;
mod utils;

fn main() {
    /* let mut b = board::Board::new();
    b.read_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
        .expect("error reading fen in main");
    println!("{b}");
    let state: String = b.state.into();
    println!("{state}") */
    let m = Move::new(Pieces::BISHOP, 20, 29, MoveType::Capture, Pieces::PAWN);
    println!("{:024b}", m.data);
    println!(
        "piece: {}, from:{}, to:{}, type:{:?}, target:{}",
        PieceNames::FULL[m.piece()],
        SQUARE_NAMES[m.from()],
        SQUARE_NAMES[m.to()],
        m.move_type(),
        PieceNames::FULL[m.target_piece()],
    );
}
