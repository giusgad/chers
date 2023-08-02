// find position of ones in binary representation of given u64
pub fn find_ones(input: u64) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 0..8 {
        if input >> i & 1 == 1 {
            res.push(i)
        }
    }
    res
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
