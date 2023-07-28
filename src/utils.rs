pub fn find_ones(input: u64) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 0..8 {
        if input >> i & 1 == 1 {
            res.push(i)
        }
    }
    res
}
