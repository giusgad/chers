// #![allow(dead_code, unused_variables)]

mod board;
mod defs;
mod engine;
mod eval;
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
