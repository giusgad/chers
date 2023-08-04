#![allow(dead_code, unused_variables)]

use moves::movegenerator::MoveGenerator;
use utils::print_bb;

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
    /* let m = Move::new(Pieces::BISHOP, 20, 29, MoveType::Capture, Pieces::PAWN);
    println!("{:024b}", m.data);
    println!(
        "piece: {}, from:{}, to:{}, type:{:?}, target:{}",
        PieceNames::FULL[m.piece()],
        SQUARE_NAMES[m.from()],
        SQUARE_NAMES[m.to()],
        m.move_type(),
        PieceNames::FULL[m.target_piece()],
    ); */

    let lm = MoveGenerator::new();
    /* for (i, bb) in lm.king.iter().enumerate() {
        print_bb(bb);
        println!("{i}^")
    } */
}
