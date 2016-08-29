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
