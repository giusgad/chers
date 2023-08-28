#![allow(dead_code, unused_variables)]

mod board;
mod defs;
mod engine;
mod eval;
mod moves;
mod search;
mod uci;
mod utils;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.start();
}

/* fn main() {
    use crate::moves::MoveGenerator;
    use board::Board;

    let mut b = Board::new();
    b.read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("error reading fen in main");
    // println!("{b}");

    let mg = MoveGenerator::new();
    /* let mut i = 0;
    while i < 20 {
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
        // LOOP TO MANUALLY INPUT MOVES
        println!("{b}\n");
        let legal = mg.get_all_legal_moves(&b);
        for i in 0..legal.index {
            print!("{}:{}, ", i, legal.list[i]);
        }
        println!();
        let mut n = String::new();
        let io = std::io::stdin();
        io.read_line(&mut n).unwrap();
        if let Ok(n) = n.split('\n').next().unwrap().parse::<usize>() {
            let legal = b.make_move(legal.list[n], &mg);
            if !legal {
                println!("ILLEGAL MOVE");
                continue;
            }
        } else {
            b.unmake()
        }
        dbg!(&b.state);
    }

    /* let mut finished = false;
    while !finished {
        // LOOP to make the engine play itself
        use rand::{thread_rng, Rng};
        println!("{b}\n");
        /* println!(
            "white:{} black:{}",
            b.state.material[Colors::WHITE],
            b.state.material[Colors::BLACK]
        );
        dbg!(b.state); */

        let mut rng = thread_rng();
        let legal_moves = mg.get_all_legal_moves(&b);
        let mut legal_moves = Vec::from_iter(legal_moves.list[0..legal_moves.index].iter());
        let mut move_made = false;
        while !move_made {
            if legal_moves.len() == 0 {
                finished = true;
                break;
            }
            let i = rng.gen_range(0..legal_moves.len());
            move_made = b.make_move(*legal_moves[i], &mg);

            legal_moves.remove(i);
        }
        if b.state.halfmove_count >= 40 {
            println!("draw");
            finished = true;
        }
        println!()
    }
    println!(
        "{} wins",
        if b.state.active_color ^ 1 == Colors::WHITE {
            "white"
        } else {
            "black"
        }
    ); */

    /* let mg = MoveGenerator::new();
    let moves = mg.get_all_legal_moves(&b);
    let mut i = 0;
    while i < moves.index {
        println!("{}", moves.list[i]);
        i += 1;
    } */
} */
