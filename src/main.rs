// #![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod engine;
mod moves;
mod utils;

use moves::MoveGenerator;

fn main() {
    let mut b = board::Board::new();
    b.read_fen("r1bqkbnr/pp3ppp/2p5/4p3/3pPQ1P/n2PB3/PPPN1PP1/R3KBNR w KQkq - 3 9")
        .expect("error reading fen in main");
    println!("{b}");

    let mg = MoveGenerator::new();
    let moves = mg.get_all_legal_moves(&b);
    let mut i = 0;
    while i < moves.index {
        dbg!(moves.list[i]);
        i += 1;
    }
}
