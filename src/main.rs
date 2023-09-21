// #![allow(dead_code, unused_variables)]
#![allow(clippy::from_over_into, clippy::needless_range_loop)]

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
}
