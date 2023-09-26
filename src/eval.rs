pub mod defs;
pub mod psqt;

use crate::{
    board::{defs::Pieces, Board},
    defs::Colors,
};

use self::psqt::{FLIP, KING_ENDGAME, PSQTS};

pub fn evaluate(b: &Board) -> i16 {
    let w_material = b.state.material[Colors::WHITE];
    let b_material = b.state.material[Colors::BLACK];

    let mut w_psqt = b.state.psqt[Colors::WHITE];
    let mut b_psqt = b.state.psqt[Colors::BLACK];

    if b.is_endgame() {
        let w_king_index = FLIP[b.king_square(Colors::WHITE)];
        let b_king_index = b.king_square(Colors::BLACK);
        // cancel the value from the normal king psqt
        w_psqt -= PSQTS[Pieces::KING][w_king_index];
        b_psqt -= PSQTS[Pieces::KING][b_king_index];
        // apply the endgame psqt
        w_psqt += KING_ENDGAME[w_king_index];
        b_psqt += KING_ENDGAME[b_king_index];
    }

    let eval = (w_material as i16 + w_psqt) - (b_material as i16 + b_psqt);

    if b.state.active_color == Colors::BLACK {
        -eval
    } else {
        eval
    }
}
