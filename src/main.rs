// #![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod engine;
mod moves;
mod utils;

use moves::MoveGenerator;

fn main() {
    let mut b = board::Board::new();
    b.read_fen("2kr1b1r/ppp1nppp/3p1q1R/4pb2/3P4/n2BPN2/PPPN1PPP/R1BQK3 b Q - 0 1")
        .expect("error reading fen in main");
    println!("{b}");
    dbg!(b.state.castling);

    /* let mg = MoveGenerator::new();
    let mut i = 0;
    while i < 100 {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let legal = mg.get_all_legal_moves(&b);
        let m = rng.gen_range(0..legal.index);
        let m = legal.list[m];
        b.make_move(m);
        println!("{b}\n");
        i += 1;
    } */
    let mg = MoveGenerator::new();
    let moves = mg.get_all_legal_moves(&b);
    let mut i = 0;
    while i < moves.index {
        dbg!(moves.list[i]);
        i += 1;
    }
}
