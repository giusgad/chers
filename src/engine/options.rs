// the EngineOption enum is used to pass options around the threads while
// the Options structs saves the values

use crate::defs::ErrFatal;

use super::Engine;

#[derive(Debug, PartialEq)]
pub enum EngineOption {
    Hash(Option<usize>),
}

pub struct Options {
    pub hash_size: usize,
}

impl Options {
    pub fn new() -> Self {
        Self {
            // default values
            hash_size: 32,
        }
    }
}

impl Engine {
    pub fn set_option(&mut self, opt: EngineOption) {
        use EngineOption::*;
        match opt {
            Hash(val) => {
                if let Some(val) = val {
                    self.options.hash_size = val;
                    self.tt.lock().expect(ErrFatal::LOCK).resize(val);
                }
            }
        }
    }
}
