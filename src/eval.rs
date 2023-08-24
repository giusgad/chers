use crate::{board::Board, defs::Colors};

pub fn evaluate(b: &Board) -> i16 {
    let white_mat = b.state.material[Colors::WHITE];
    let black_mat = b.state.material[Colors::BLACK];

    let eval = white_mat as i16 - black_mat as i16;

    if b.state.active_color == Colors::BLACK {
        -eval
    } else {
        eval
    }
}
