use super::Engine;
use crate::uci::consts::UciData;
use std::sync::mpsc;

impl Engine {
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel::<UciData>();

        self.uci.start(tx);
        while !self.stop {
            match rx.recv() {
                Ok(c) => self.exec_command(c),
                Err(e) => panic!("Error in communication thread recv: {e}"),
            }
        }
    }

    fn exec_command(&self, command: UciData) {
        dbg!(command);
    }
}
