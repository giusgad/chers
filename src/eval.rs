pub mod defs;
pub mod psqt;

use crate::{board::Board, defs::Colors};

pub fn evaluate(b: &Board) -> i16 {
    let w_material = b.state.material[Colors::WHITE];
    let b_material = b.state.material[Colors::BLACK];

    let w_psqt = b.state.psqt[Colors::WHITE];
    let b_psqt = b.state.psqt[Colors::BLACK];

    let eval = (w_material as i16 + w_psqt) - (b_material as i16 + b_psqt);

    if b.state.active_color == Colors::BLACK {
        -eval
    } else {
        eval
    }
}
