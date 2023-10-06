pub mod defs;
pub mod psqt;

use crate::{
    board::{defs::Pieces, Board},
    defs::{Colors, NrOf},
};

// king,queen,rook,bishop,knight,pawn
const PHASE_VALUES: [i16; NrOf::PIECE_TYPES] = [0, 4, 2, 1, 1, 0];
const PHASE_CONST: i16 = 24;
pub fn game_phase(b: &Board) -> (i16, i16) {
    let mut val = 0;
    for color in 0..Colors::BOTH {
        for (piece, bb) in b.piece_bbs[color]
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != Pieces::NONE)
        {
            val += bb.count_ones() as i16 * PHASE_VALUES[piece];
        }
    }
    let mg = std::cmp::min(PHASE_CONST, val);
    let eg = PHASE_CONST - mg;
    (mg, eg)
}

pub fn evaluate(b: &Board) -> i16 {
    let w_material = b.state.material[Colors::WHITE];
    let b_material = b.state.material[Colors::BLACK];

    let w_psqt_mg = b.state.psqt_mg[Colors::WHITE];
    let b_psqt_mg = b.state.psqt_mg[Colors::BLACK];
    let w_psqt_eg = b.state.psqt_eg[Colors::WHITE];
    let b_psqt_eg = b.state.psqt_eg[Colors::BLACK];

    let (mg_phase, eg_phase) = game_phase(b);

    let w_psqt = w_psqt_mg * mg_phase + w_psqt_eg * eg_phase;
    let b_psqt = b_psqt_mg * mg_phase + b_psqt_eg * eg_phase;

    let eval =
        (w_material as i16 + w_psqt / PHASE_CONST) - (b_material as i16 + b_psqt / PHASE_CONST);

    if b.state.active_color == Colors::BLACK {
        -eval
    } else {
        eval
    }
}

#[cfg(test)]
mod test {
    use super::evaluate;
    use crate::board::Board;

    #[test]
    fn evaluation_simmetry() {
        let mut b = Board::new();
        for (w_fen, b_fen) in [
            (
                "r1k2b2/ppp2pp1/4qn2/8/8/2NQ4/1PP2PP1/2B2K1R w - - 0 1",
                "r1k2b2/ppp2pp1/4qn2/8/8/2NQ4/1PP2PP1/2B2K1R b - - 0 1",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
        ] {
            b.read_fen(w_fen).unwrap();
            let w_eval = evaluate(&b);
            b.read_fen(b_fen).unwrap();
            let b_eval = evaluate(&b);
            assert_eq!(w_eval, -b_eval);
        }
    }
}
