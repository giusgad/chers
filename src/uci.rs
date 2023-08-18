pub mod consts;
mod parser;

use self::consts::UciData;
use std::{
    sync::{mpsc::Sender, Arc},
    thread,
};

pub struct Uci {
    stop: Arc<bool>,
}

impl Uci {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(false),
        }
    }
    pub fn start(&mut self, tx: Sender<UciData>) {
        // self.tx = Some(tx);
        let stop = Arc::clone(&self.stop);
        thread::spawn(move || {
            while !*stop {
                let mut buf = String::new();
                let io = std::io::stdin();
                io.read_line(&mut buf).expect("Stdin error in uci");
                let command = Self::commands_from_string(buf);
                tx.send(command).expect("Error sending uci command");
            }
        });
    }
}
