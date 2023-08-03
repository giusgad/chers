#![allow(dead_code, unused_variables)]

use search::consts::Move;

use crate::{
    board::consts::{square_by_name, PieceNames, Pieces, SQUARE_NAMES},
    consts::Piece,
};

mod board;
mod consts;
mod search;
mod utils;

fn main() {
    /* let mut b = board::Board::new();
    b.read_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
        .expect("error reading fen in main");
    println!("{b}");
    let state: String = b.state.into();
    println!("{state}") */
    let mut mov: u32 = 0;

    let m = Move {
        // data: 0b101_000_001011_101100_001,
        data: mov,
    };
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
