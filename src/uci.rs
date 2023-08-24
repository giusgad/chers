pub mod defs;
mod parser;

use crate::defs::Info;

use std::{
    sync::{mpsc::Sender, Arc},
    thread,
};

pub struct Uci {
    pub stop: Arc<bool>,
}

impl Uci {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(false),
        }
    }

    pub fn init(&mut self, tx: Sender<Info>) {
        // self.tx = Some(tx);
        let stop = Arc::clone(&self.stop);
        thread::spawn(move || {
            while !*stop {
                let mut buf = String::new();
                let io = std::io::stdin();
                io.read_line(&mut buf).expect("Stdin error in uci");
                let command = Self::commands_from_string(buf);
                tx.send(Info::Uci(command))
                    .expect("Error sending uci command");
            }
        });
    }

    pub fn output(s: impl std::fmt::Display) {
        println!("{s}");
    }
}
