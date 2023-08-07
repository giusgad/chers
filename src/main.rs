#![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod engine;
mod moves;
mod utils;

use consts::Colors;
use moves::MoveGenerator;
use utils::print_bb;

use crate::{
    board::consts::{PieceNames, Pieces, SQUARE_NAMES},
    moves::consts::{Move, MoveType},
};

fn main() {
    let mut b = board::Board::new();
    b.read_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1")
        .expect("error reading fen in main");
    println!("{b}");
    // let state = b.pieces;
    // println!("{:?}", state);
    /* let m = Move::new(
        Pieces::BISHOP,
        20,
        29,
        MoveType::Capture,
        Pieces::PAWN,
        None,
    );
    println!("{:024b}", m.data);
    println!(
        "piece: {}, from:{}, to:{}, type:{:?}, captured:{}, promotion:{}, promoted to:{}",
        PieceNames::FULL[m.piece()],
        SQUARE_NAMES[m.from()],
        SQUARE_NAMES[m.to()],
        m.move_type(),
        PieceNames::FULL[m.captured_piece()],
        m.is_promotion(),
        PieceNames::FULL[m.promoted_to()]
    ); */

    let lm = MoveGenerator::new();
    let list = lm.get_all_legal_moves(&b);
    let mut i = 0;
    while i < list.index {
        dbg!(list.list[i]);
        i += 1;
    }
}
