extern crate egg_mode;
#[macro_use]
extern crate clap;

mod outcome;

pub mod ops;
pub mod options;

pub use outcome::Outcome;

use std::process::exit;
use std::io::stderr;


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = options::Options::parse();
    println!("{:#?}", opts);

    let err = match opts.subsystem {
        options::Subsystem::Init { force } => ops::init(&opts.config_dir, force),
        options::Subsystem::AddUser => Outcome::NoError,
    };
    err.print_error(&mut stderr());
    err.exit_value()
}
