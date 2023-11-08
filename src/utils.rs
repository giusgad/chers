use crate::{
    board::defs::Pieces,
    defs::{Bitboard, NrOf, Piece, MASK_8},
};

// find position of ones in binary representation of given u64
pub mod bit_ops {
    pub fn find_ones_u8(input: u64) -> Vec<usize> {
        let mut res = Vec::new();
        for i in 0..8 {
            if input >> i & 1 == 1 {
                res.push(i);
            }
        }
        res
    }

    pub fn one_indexes(bb: u64) -> Vec<u64> {
        let mut res = Vec::new();
        for i in 0..64 {
            if (bb >> i) & 1 == 1 {
                res.push(i);
            }
        }
        res
    }

    pub struct BitIterHelper {
        num: u64,
    }

    impl Iterator for BitIterHelper {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let pos = self.num.trailing_zeros() as usize;
            if pos < 64 {
                self.num ^= 1 << pos;
                Some(pos)
            } else {
                None
            }
        }
    }

    pub trait BitIterator {
        fn bit_iter(&self) -> BitIterHelper;
    }

    impl BitIterator for u64 {
        fn bit_iter(&self) -> BitIterHelper {
            BitIterHelper { num: *self }
        }
    }
}

// used to compare bytes in const
const fn const_bytes_equal(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
        i += 1;
    }
    true
}

// const fn used to compare two &str
pub const fn const_str_equal(lhs: &str, rhs: &str) -> bool {
    const_bytes_equal(lhs.as_bytes(), rhs.as_bytes())
}

pub fn add_square_i8(sq: usize, i: i8) -> Option<usize> {
    if i < 0 {
        sq.checked_sub(i.unsigned_abs() as usize)
    } else {
        let res = sq + i as usize;
        if res < NrOf::SQUARES {
            Some(res)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn print_bb(bb: Bitboard) {
    let mut ranks = [0; 8];
    for (i, rank) in ranks.iter_mut().enumerate() {
        *rank = (bb >> (8 * i)) & MASK_8;
    }
    for rank in (0..8).rev() {
        for bit in 0..8 {
            print!("|{}", ranks[rank] >> bit & 1);
        }
        println!("|");
    }
}

pub fn remove_from_vec<T: PartialEq>(v: &mut Vec<T>, r: &[T]) {
    for val in r {
        if let Some(i) = v.iter().position(|x| x == val) {
            v.remove(i);
        }
    }
}

pub const fn piece_from_char(c: char) -> Result<Piece, ()> {
    let c = match c {
        c if c > 'A' && c < 'Z' => char::from_u32(c as u32 + 32),
        c => Some(c),
    };
    if let Some(c) = c {
        match c {
            'k' => Ok(Pieces::KING),
            'q' => Ok(Pieces::QUEEN),
            'r' => Ok(Pieces::ROOK),
            'b' => Ok(Pieces::BISHOP),
            'n' => Ok(Pieces::KNIGHT),
            'p' => Ok(Pieces::PAWN),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}
