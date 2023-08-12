// #![allow(dead_code, unused_variables)]

mod board;
mod consts;
mod engine;
mod moves;
mod utils;

use moves::MoveGenerator;

fn main() {
    let mut b = board::Board::new();
    b.read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("error reading fen in main");
    // println!("{b}");

    let mg = MoveGenerator::new();
    let mut i = 0;
    /* while i < 20 {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let legal = mg.get_all_legal_moves(&b);
        let m = rng.gen_range(0..legal.index);
        let m = legal.list[m];
        b.make_move(m);
        println!("{b}\n");
        dbg!(&b.state);
        i += 1;
    } */
    loop {
        println!("{b}\n");
        let legal = mg.get_all_legal_moves(&b);
        for i in 0..legal.index {
            print!("{}:{}, ", i, legal.list[i]);
        }
        println!();
        let mut n = String::new();
        let io = std::io::stdin();
        io.read_line(&mut n).unwrap();
        let n: usize = n.split('\n').next().unwrap().parse().unwrap();
        b.make_move(legal.list[n]);
        dbg!(&b.state);
    }
    /* let mg = MoveGenerator::new();
    let moves = mg.get_all_legal_moves(&b);
    let mut i = 0;
    while i < moves.index {
        println!("{}", moves.list[i]);
        i += 1;
    } */
}
