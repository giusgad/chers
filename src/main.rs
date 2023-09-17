// #![allow(dead_code, unused_variables)]

mod board;
mod defs;
mod engine;
mod eval;
mod find_magics;
mod moves;
mod search;
mod tests;
mod uci;
mod utils;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.start();

    /* let mg = MoveGenerator::new();
    let mut b = board::Board::new();
    b.read_fen("8/1k6/8/3r1R2/8/8/3K4/5R2 w - - 0 1").unwrap();
    println!("{b}");
    let legal = mg.get_all_legal_moves(&b, false);
    for m in legal.iter() {
        // dbg!(m);
    } */
}
