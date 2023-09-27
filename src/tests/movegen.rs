#![allow(unused_imports, dead_code)]
use std::time::Instant;

use crate::{
    board::Board,
    defs::START_FEN,
    moves::{defs::MoveType, MoveGenerator},
};

// positions and results are from https://www.chessprogramming.org/Perft_Results

struct DbgRefs<'a> {
    timer: Instant,
    board: &'a mut Board,
    mg: &'a MoveGenerator,
    legal_moves: u64,
    captures: u64,
}
// search function with no optimizations for testing the move generation
fn search(refs: &mut DbgRefs, depth: u8) {
    if depth == 0 {
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
}

// this function sets up the given fen string, searches to the given depth and panics
// if the number of legal moves found is different from the argument one
fn do_test(fen: &str, depth: u8, moves: u64, captures: Option<u64>) {
    let mut b = Board::new();
    let mut mg = MoveGenerator::default();
    mg.init();
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
    if let Some(captures) = captures {
        assert_eq!(refs.captures, captures);
    }
}

#[test]
fn movegen_startpos() {
    let startpos = [
        (1, 20, 0),
        (2, 400, 0),
        (3, 8902, 34),
        (4, 197281, 1576),
        (5, 4865609, 82719),
    ];
    let time = Instant::now();
    for (depth, moves, captures) in startpos {
        do_test(START_FEN, depth, moves, Some(captures))
    }
    println!("Startpos time: {}ms", time.elapsed().as_millis());
}

#[test]
fn movegen_kiwipete() {
    // depth, moves, captures
    let kiwipete = [
        (1, 48, 8),
        (2, 2039, 351),
        (3, 97862, 17102),
        (4, 4085603, 757163),
    ];
    let time = Instant::now();
    for (depth, moves, captures) in kiwipete {
        do_test(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            depth,
            moves,
            Some(captures),
        )
    }
    println!("Kiwipete time: {}ms", time.elapsed().as_millis());
}

#[test]
fn movegen_other_positions() {
    let time = Instant::now();
    let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    do_test(fen, 3, 62379, None);
    do_test(fen, 4, 2103487, None);

    let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ";
    do_test(fen, 3, 89890, None);
    do_test(fen, 4, 3894594, None);

    println!("Other positions time: {}ms", time.elapsed().as_millis())
}
