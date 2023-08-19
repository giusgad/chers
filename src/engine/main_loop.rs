use super::Engine;
use crate::{search::consts::SearchControl, uci::consts::UciData};
use std::sync::{mpsc, Arc};

impl Engine {
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel::<UciData>();

        self.uci.init(tx);
        self.search
            .init(Arc::clone(&self.board), Arc::clone(&self.mg));

        self.search.send(SearchControl::Start); // : remove

        while !self.stop {
            match rx.recv() {
                Ok(c) => self.exec_command(c),
                Err(e) => panic!("Error in communication thread recv: {e}"),
            }
        }
    }

    fn exec_command(&self, command: UciData) {
        dbg!(&command);
        match command {
            UciData::Go(_) => self.search.send(SearchControl::Start),
            _ => (),
        }
    }
}
