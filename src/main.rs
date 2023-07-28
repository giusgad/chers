#![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod utils;

fn main() {
    let b = board::Board::new();
    println!("{}", b);
}
