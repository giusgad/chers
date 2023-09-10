#![allow(unused_imports, dead_code)]
use std::time::Instant;

use crate::{
    board::Board,
    defs::START_FEN,
    moves::{defs::MoveType, MoveGenerator},
};

struct DbgRefs<'a> {
    timer: Instant,
    board: &'a mut Board,
    mg: &'a MoveGenerator,
    legal_moves: u64,
    captures: u64,
}
// search function with no optimizations for testing the move generation
fn search(refs: &mut DbgRefs, depth: u8) {
    if depth <= 0 {
        return;
    }

    let mut legal_moves = 0;
    let mut captures = 0;

    let moves = refs.mg.get_all_legal_moves(refs.board, false);
    for m in moves.iter() {
        let legal = refs.board.make_move(*m, refs.mg);
        if !legal {
            continue;
        }

        search(refs, depth - 1);

        refs.board.unmake();
        legal_moves += 1;
        if m.move_type() == MoveType::Capture {
            captures += 1;
        }
    }
    if depth == 1 {
        refs.legal_moves += legal_moves;
        refs.captures += captures;
    }
    if legal_moves == 0 {
        return;
    }
}

// this function sets up the given fen string, searches to the given depth and panics
// if the number of legal moves found is different from the argument one
fn do_test(fen: &str, depth: u8, moves: u64, captures: u64) {
    let mut b = Board::new();
    let mg = MoveGenerator::new();
    b.read_fen(fen).unwrap();
    let mut refs = DbgRefs {
        board: &mut b,
        mg: &mg,
        legal_moves: 0,
        captures: 0,
        timer: Instant::now(),
    };
    search(&mut refs, depth);
    println!(
        "FEN: {fen} DEPTH: {depth} TIME: {}ms",
        refs.timer.elapsed().as_millis()
    );
    assert_eq!(refs.legal_moves, moves);
    assert_eq!(refs.captures, captures);
}

#[test]
fn startpos() {
    let startpos = [
        (1, 20, 0),
        (2, 400, 0),
        (3, 8902, 34),
        (4, 197281, 1576),
        (5, 4865609, 82719),
    ];
    let time = Instant::now();
    for (depth, moves, captures) in startpos {
        do_test(START_FEN, depth, moves, captures)
    }
    println!("Startpos time: {}ms", time.elapsed().as_millis());
}

#[test]
fn kiwipete() {
    // depth, moves, captures
    let kiwipete = [
        (1, 48, 8),
        (2, 2039, 351),
        (3, 97862, 17102),
        (4, 4085603, 757163),
        // (5, 193690690, 35043416), // TODO: enable later, too slow now (20 minutes wasn't enough)
    ];
    let time = Instant::now();
    for (depth, moves, captures) in kiwipete {
        do_test(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            depth,
            moves,
            captures,
        )
    }
    println!("Kiwipete time: {}ms", time.elapsed().as_millis());
}
