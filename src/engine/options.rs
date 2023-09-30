#![allow(clippy::redundant_closure_call)]

use super::Engine;
use crate::{
    defs::ErrFatal,
    engine::transposition::{SearchData, TT},
    uci::Uci,
};
use std::sync::MutexGuard;

// This trait is used to convert a type name from rust to what is used in the uci convention
trait UciType {
    fn uci_type() -> String;
}
impl UciType for bool {
    fn uci_type() -> String {
        String::from("check")
    }
}
impl UciType for usize {
    fn uci_type() -> String {
        String::from("spin")
    }
}

// this simple macro extracts a value from a string given the type
// and it's used to parse option values into rust types
macro_rules! extract_val {
    (usize, $s:expr) => {{
        if let Ok(val) = $s.parse::<usize>() {
            Ok(val)
        } else {
            Err(())
        }
    }};
    (bool, $s:expr) => {{
        Ok::<bool, ()>($s.to_lowercase() == "true")
    }};
}

/* Options are set with the use of this macro, which takes in:
*  - name of the option in CamelCase and snake_case: the name in camel case is what is shown to the
*                                                    gui and is used to set the option
*  - type in rust for the option
*  - default value
*  - optional: closure for extra steps to perform when setting the options
*              the closure can be adapted to have more arguments, every options will only use the
*              needed ones
* With this arguments macro magic will do the rest and set up everything needed. Yay!
*/
macro_rules! define_options {
    {$($camel_name:ident, $snake_name:ident, $type:tt, $default:literal  $(,$extra:expr)? $(,$extra_uci:literal)?)
        *} => {
        // enum obtained from parsing uci and containing the value to set
        #[derive(Debug, PartialEq)]
        pub enum EngineOption {
            $($camel_name($type),)*
        }


        // struct in the engine that hold the current option values
        #[derive(Debug)]
        pub struct Options {
            $(pub $snake_name: $type,)*
        }

        impl Default for Options {
            fn default() -> Self {
                Self {
                    $($snake_name:$default,)*
                }
            }
        }

        // when the uci command is called, the option names and their types are listed to the gui
        impl Uci {
            pub fn show_options() {
                $(print!("option name {} type {} default {}",stringify!($camel_name),<$type>::uci_type(),$default);$(print!(" {}", $extra_uci);)? println!();)*
            }
        }

        // function to obtain an enum variant from a string
        impl EngineOption {
            pub fn from_string(name: &str, val: &str) -> Result<Self,()> {
                match name {
                    $(stringify!($camel_name) => {
                        if let Ok(val) = extract_val!($type, val) {
                            Ok(Self::$camel_name(val))
                        } else {
                            Err(())
                        }
                    })*
                    _ => Err(())
                }
            }
        }

        impl Engine {
            pub fn set_option(&mut self, opt: EngineOption) {
                use EngineOption::*;
                let mut opts = self.options.lock().expect(ErrFatal::LOCK);
                match opt {
                    $($camel_name(val) => {
                        opts.$snake_name = val;
                        // the extra block is optional and contains actions that need to be
                        // performed when the option is set (i.e. resizing the tt)
                        $( {
                        let mut tt = self.tt.lock().expect(ErrFatal::LOCK);
                        $extra(&mut tt,val)
                        } )?
                    })*
                }
            }
        }
    };
}

// This is the actual call to the macro, to add a new option simply add it here
define_options! {
    Hash,hash_size,usize,32,|tt:&mut MutexGuard<TT<SearchData>>,val| {tt.resize(val);},"min 1 max 32768"
    EarlyStop,early_stop,bool,true
    DbgUnicode,dbg_unicode,bool,true
}
