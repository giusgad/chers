#![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod utils;

fn main() {
    let mut b = board::Board::new();
    b.read_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    println!("{b}");
    let state: String = b.state.into();
    println!("{state}")
}
