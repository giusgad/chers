// #![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod engine;
mod moves;
mod utils;

use moves::MoveGenerator;

fn main() {
    let mut b = board::Board::new();
    b.read_fen("rnbqkbnr/pppp1p1p/6p1/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3")
        .expect("error reading fen in main");
    println!("{b}");

    let lm = MoveGenerator::new();
    let list = lm.get_all_legal_moves(&b);
    let mut i = 0;
    while i < list.index {
        dbg!(list.list[i]);
        i += 1;
    }
}
