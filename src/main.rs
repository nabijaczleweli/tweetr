extern crate rustc_serialize;
extern crate egg_mode;
#[macro_use]
extern crate clap;
extern crate toml;

mod outcome;

pub mod ops;
pub mod util;
pub mod options;

pub use outcome::Outcome;

use std::process::exit;
use std::io::{stdin, stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = options::Options::parse();
    println!("{:#?}", opts);

    let err = match opts.subsystem {
        options::Subsystem::Init { force } => {
            match ops::init::verify(&opts.config_dir, force) {
                Ok(pb) => {
                    println!("{:?}", pb);
                    let stdin = stdin();
                    let mut lock = stdin.lock();

                    let data = ops::init::get_data(&mut lock, &mut stdout());
                    data.write(&pb);
                    Outcome::NoError
                }
                Err(out) => out,
            }
        }
        options::Subsystem::AddUser => Outcome::NoError,
    };
    err.print_error(&mut stderr());
    err.exit_value()
}
