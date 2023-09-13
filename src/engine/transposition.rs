use crate::{defs::ZobristHash, moves::defs::Move};

const BUCKET_ENTIRES: usize = 4;

#[derive(Clone, Copy)]
enum EvalType {
    Exact,
    Alpha,
    Beta,
}

#[derive(Clone, Copy)]
pub struct SearchData {
    best_move: Move,
    depth: u8,
    eval: i16,
    eval_type: EvalType,
    zobrist_hash: ZobristHash,
}

impl SearchData {
    fn new() -> Self {
        Self {
            best_move: Move { data: 0 },
            depth: 0,
            eval: 0,
            eval_type: EvalType::Exact,
            zobrist_hash: 0,
        }
    }
}

// A Bucket contains BUCKET_SIZE entires that would be mapped to the same index in the tt
#[derive(Clone, Copy)]
pub struct Bucket {
    data: [SearchData; BUCKET_ENTIRES],
}

impl Bucket {
    fn new() -> Self {
        Self {
            data: [SearchData::new(); BUCKET_ENTIRES],
        }
    }

    // insert the data in the bucket and return whether the count of used entries needs to be increased.
    // If the data is not inserted there was higher quality data in the bucket
    fn insert(&mut self, data: SearchData) -> bool {
        let mut min_depth = data.depth;
        let mut min_depth_i: Option<usize> = None;

        // search for the entry with the smallest depth that will be replaced
        for (i, entry) in self.data.iter().enumerate() {
            if entry.depth < min_depth {
                min_depth = entry.depth;
                min_depth_i = Some(i);
            }
        }

        // if all entries had higher depth than the new data's then it doesn't get inserted
        if let Some(i) = min_depth_i {
            // if the hash is 0 the entry was never used so the counter has to be increased
            let new = self.data[i].zobrist_hash == 0;

            self.data[i] = data;

            return new;
        }
        false
    }

    fn get(&self, hash: ZobristHash) -> Option<SearchData> {
        for entry in self.data {
            if entry.zobrist_hash == hash {
                return Some(entry);
            }
        }
        None
    }
}

pub struct TT {
    data: Vec<Bucket>,
    megabytes: usize,
    total_entries: usize,
    total_buckets: usize,
    used_entries: usize,
}

impl TT {
    pub fn new(megabytes: usize) -> Self {
        let (total_buckets, total_entries) = Self::calculate_sizes(megabytes);
        TT {
            megabytes,
            data: vec![Bucket::new(); total_buckets],
            total_entries,
            total_buckets,
            used_entries: 0,
        }
    }

    pub fn resize(&mut self, megabytes: usize) {
        let (total_buckets, total_entries) = Self::calculate_sizes(megabytes);
        self.data = vec![Bucket::new(); total_buckets];
        self.megabytes = megabytes;
        self.used_entries = 0;
        self.total_buckets = total_buckets;
        self.total_entries = total_entries;
    }

    pub fn save(&mut self, data: SearchData) {
        let index = self.calculate_index(data.zobrist_hash);
        debug_assert!(index < self.total_buckets);
        let new_entry = self.data[index].insert(data);
        if new_entry {
            self.used_entries += 1;
        }
    }

    pub fn get(&self, hash: ZobristHash) -> Option<SearchData> {
        let index = self.calculate_index(hash);
        self.data[index].get(hash)
    }
}

// utility functions
const MB: usize = 1048576;
impl TT {
    fn calculate_index(&self, hash: ZobristHash) -> usize {
        (hash as usize) % self.total_buckets
    }

    fn calculate_sizes(megabytes: usize) -> (usize, usize) {
        let bucket_size = std::mem::size_of::<Bucket>();
        let buckets = MB / bucket_size * megabytes;
        let entries = buckets * BUCKET_ENTIRES;
        (buckets, entries)
    }
}

impl std::fmt::Debug for TT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TT")
            .field("megabytes", &self.megabytes)
            .field("total_entries", &self.total_entries)
            .field("total_buckets", &self.total_buckets)
            .field("used_entries", &self.used_entries)
            .finish()
    }
}