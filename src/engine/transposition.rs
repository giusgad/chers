use crate::{defs::ZobristHash, moves::defs::Move};

const BUCKET_ENTIRES: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EvalType {
    Exact,
    Alpha,
    Beta,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SearchData {
    pub best_move: Move,
    pub depth: u8,
    pub eval: i16,
    pub eval_type: EvalType,
    pub zobrist_hash: ZobristHash,
}

impl Default for SearchData {
    fn default() -> Self {
        Self {
            best_move: Move::default(),
            depth: 0,
            eval: 0,
            eval_type: EvalType::Exact,
            zobrist_hash: 0,
        }
    }
}

impl SearchData {
    pub fn get_values(&self, alpha: i16, beta: i16, depth: u8) -> (Option<i16>, Move) {
        let mut eval = None;
        if self.depth >= depth {
            match self.eval_type {
                EvalType::Exact => eval = Some(self.eval),
                EvalType::Alpha => {
                    if self.eval <= alpha {
                        eval = Some(alpha)
                    }
                }
                EvalType::Beta => {
                    if self.eval >= beta {
                        eval = Some(beta)
                    }
                }
            }
        }
        (eval, self.best_move)
    }
}

impl TTData for SearchData {
    fn key(&self) -> u64 {
        self.zobrist_hash
    }

    fn depth(&self) -> u8 {
        self.depth
    }
}

pub trait TTData {
    fn key(&self) -> u64;
    fn depth(&self) -> u8;
}

// A Bucket contains BUCKET_SIZE entires that would be mapped to the same index in the tt
#[derive(Clone, Copy, Default)]
pub struct Bucket<T: TTData + Default + Copy + Clone> {
    data: [T; BUCKET_ENTIRES],
}

impl<T: TTData + Default + Copy + Clone> Bucket<T> {
    // insert the data in the bucket and return whether the count of used entries needs to be increased.
    // If the data is not inserted there was higher quality data in the bucket
    fn insert(&mut self, data: T) -> bool {
        let mut min_priority = data.depth();
        let mut min_priority_i: Option<usize> = None;

        // search for the entry with the smallest depth that will be replaced
        for (i, entry) in self.data.iter().enumerate() {
            if entry.depth() <= min_priority {
                min_priority = entry.depth();
                min_priority_i = Some(i);
            }
        }

        // if all entries had higher depth than the new data's then it doesn't get inserted
        if let Some(i) = min_priority_i {
            // if the hash is 0 the entry was never used so the counter has to be increased
            let new = self.data[i].key() == 0;

            self.data[i] = data;

            return new;
        }
        false
    }

    fn get(&self, key: u64) -> Option<T> {
        self.data.into_iter().find(|entry| entry.key() == key)
    }
}

pub struct TT<T: TTData + Default + Copy + Clone> {
    data: Vec<Bucket<T>>,
    megabytes: usize,
    total_entries: usize,
    total_buckets: usize,
    used_entries: usize,
}

impl<T: TTData + Default + Copy + Clone> TT<T> {
    pub fn new(megabytes: usize) -> Self {
        let (total_buckets, total_entries) = Self::calculate_sizes(megabytes);
        TT {
            megabytes,
            data: vec![Bucket::default(); total_buckets],
            total_entries,
            total_buckets,
            used_entries: 0,
        }
    }

    pub fn resize(&mut self, megabytes: usize) {
        let (total_buckets, total_entries) = Self::calculate_sizes(megabytes);
        self.data = vec![Bucket::default(); total_buckets];
        self.megabytes = megabytes;
        self.used_entries = 0;
        self.total_buckets = total_buckets;
        self.total_entries = total_entries;
    }

    pub fn insert(&mut self, data: T) {
        let index = self.calculate_index(data.key());
        debug_assert!(index < self.total_buckets);
        let new_entry = self.data[index].insert(data);
        if new_entry {
            self.used_entries += 1;
        }
    }

    pub fn get(&self, hash: u64) -> Option<T> {
        let index = self.calculate_index(hash);
        self.data[index].get(hash)
    }
}

// utility functions
const MB: usize = 1048576;
impl<T: TTData + Default + Copy + Clone> TT<T> {
    fn calculate_index(&self, key: u64) -> usize {
        (key as usize) % self.total_buckets
    }

    fn calculate_sizes(megabytes: usize) -> (usize, usize) {
        let bucket_size = std::mem::size_of::<Bucket<T>>();
        let buckets = MB / bucket_size * megabytes;
        let entries = buckets * BUCKET_ENTIRES;
        (buckets, entries)
    }

    pub fn hash_full(&self) -> u16 {
        let permil = (self.used_entries as f64 / self.total_entries as f64) * 1000f64;
        permil.round() as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn tt_dbg_test() {
        #[derive(Clone, Copy, Default, PartialEq, Debug)]
        struct DbgData {
            content: u64,
            key: u64,
        }
        impl TTData for DbgData {
            fn key(&self) -> u64 {
                self.key
            }

            fn depth(&self) -> u8 {
                3
            }
        }
        let mut tt: TT<DbgData> = TT::new(16);
        let data_in = DbgData {
            content: 392,
            key: 32,
        };
        tt.insert(data_in);
        let data_out = tt.get(data_in.key());

        assert_eq!(data_in, data_out.unwrap());
    }

    #[test]
    fn tt_search_test() {
        let mut tt: TT<SearchData> = TT::new(32);
        let mut b = Board::new();
        b.read_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
        let data_in = SearchData {
            best_move: Move::default(),
            depth: 5,
            eval: 58,
            eval_type: EvalType::Beta,
            zobrist_hash: b.state.zobrist_hash,
        };

        tt.insert(data_in);
        let data_out = tt.get(b.state.zobrist_hash);

        assert_eq!(data_in, data_out.unwrap());
    }
}
