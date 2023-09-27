// the EngineOption enum is used to pass options around the threads while
// the Options structs saves the values

use crate::defs::ErrFatal;

use super::Engine;

#[derive(Debug, PartialEq)]
pub enum EngineOption {
    Hash(Option<usize>),
    EarlyStop(bool),
    DbgUnicode(bool),
}

#[derive(Debug)]
pub struct Options {
    pub hash_size: usize,
    pub early_stop: bool,
    pub dbg_unicode: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            // default values
            hash_size: 32,
            early_stop: true,
            dbg_unicode: true,
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
            DbgUnicode(val) => opts.dbg_unicode = val,
        }
    }
}
