use crate::{
    board::defs::{Pieces, SQUARE_NAMES},
    defs::{NrOf, Piece},
    moves::{magics::Magic, MoveGenerator, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE},
};
use rand::{thread_rng, Rng};

#[allow(dead_code)]
pub fn find_magics(piece: Piece) {
    let mg = MoveGenerator::new();
    let (masks, size) = match piece {
        Pieces::BISHOP => (mg.bishop_masks, BISHOP_TABLE_SIZE),
        Pieces::ROOK => (mg.rook_masks, ROOK_TABLE_SIZE),
        _ => panic!(),
    };
    let mut magics = [Magic::new(); NrOf::SQUARES];
    let mut rng = thread_rng();
    let mut end = 0;
    let mut table = vec![0; size]; // the table that will contain all the blocker-legal

    for sq in 0..NrOf::SQUARES {
        let mask = MoveGenerator::simplify_blocker(masks[sq], sq);
        let blockers = MoveGenerator::generate_blockers(mask);
        let mut legals = vec![0; blockers.len()];
        for i in 0..blockers.len() {
            legals[i] = MoveGenerator::piece_rays_bb(piece, sq, blockers[i]);
        }

        let shift = 64 - mask.count_ones() as usize;
        let offset = end;
        end = offset + (1 << mask.count_ones());

        let mut magic = Magic::new();
        magic.shift = shift;
        magic.offset = offset;

        let mut found = false;
        let mut attempts = 0;
        while !found {
            attempts += 1;
            found = true;
            magic.nr = rng.gen_range(0..u64::MAX)
                & rng.gen_range(0..u64::MAX)
                & rng.gen_range(0..u64::MAX);
            for i in 0..blockers.len() {
                let table_i = magic.get_index(blockers[i]);

                let table = &mut table[..];

                if table[table_i] == 0 {
                    // the slot is empty
                    assert!(
                        table_i <= end && table_i >= offset,
                        "{table_i},{end},{offset}"
                    );
                    table[table_i] = legals[i];
                } else {
                    // there was an index collision, so the magic number was wrong
                    found = false;
                    // clear the part of the table that is being used
                    for i in offset..end {
                        table[i] = 0;
                    }
                    break;
                }
            }
        }
        println!(
            "Magic for {} found in {} attempts: {:?}",
            SQUARE_NAMES[sq], attempts, magic
        );
        magics[sq] = magic;
    }
    assert_eq!(end, size);
}
