pub mod defs;
mod parser;
mod search_info;

use crossbeam_channel::Sender;

use crate::defs::{ErrFatal, Info};

use std::thread::{self, JoinHandle};

use self::defs::UciData;

// The Uci module reads input and sends the received commands to the engine which then handles them
#[derive(Default)]
pub struct Uci {
    pub handle: Option<JoinHandle<()>>,
}

impl Uci {
    pub fn init(&mut self, tx: Sender<Info>) {
        let h = thread::spawn(move || {
            let mut quit = false;
            while !quit {
                let mut buf = String::new();
                let io = std::io::stdin();
                io.read_line(&mut buf).expect(ErrFatal::STDIN);

                // parse input
                let command = Self::commands_from_string(buf);

                // update quit condition based on the new input
                quit = command == UciData::Quit;

                // send command to engine
                tx.send(Info::Uci(command)).expect(ErrFatal::TX_SEND);
            }
        });

        self.handle = Some(h);
    }

    pub fn output(s: impl std::fmt::Display) {
        println!("{s}");
    }
    pub fn output_err(s: impl std::fmt::Display) {
        println!("info string {s}");
    }

    pub fn show_options() {
        for option in [
            ("Hash", "spin default 32 min 1 max 8192"),
            ("EarlyStop", "check default true"),
            ("DbgUnicode", "check default true"),
        ] {
            println!("option name {} type {}", option.0, option.1);
        }
    }
}
