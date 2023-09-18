// the EngineOption enum is used to pass options around the threads while
// the Options structs saves the values

use crate::defs::ErrFatal;

use super::Engine;

#[derive(Debug, PartialEq)]
pub enum EngineOption {
    Hash(Option<usize>),
    EarlyStop(bool),
}

#[derive(Debug)]
pub struct Options {
    pub hash_size: usize,
    pub early_stop: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            // default values
            hash_size: 32,
            early_stop: true,
        }
    }
}

impl Engine {
    pub fn set_option(&mut self, opt: EngineOption) {
        use EngineOption::*;
        let mut opts = self.options.lock().expect(ErrFatal::LOCK);
        match opt {
            Hash(val) => {
                if let Some(val) = val {
                    opts.hash_size = val;
                    self.tt.lock().expect(ErrFatal::LOCK).resize(val);
                }
            }
            EarlyStop(val) => opts.early_stop = val,
        }
    }
}
