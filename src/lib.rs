//! [not-stakkr](https://github.com/nabijaczleweli/not-stakkr) is a platform that allows you to create and queue tweets to be
//! shared when YOU want. You create content when you have time and then use FOSS and NOT pay whatever-ridiculous amount of $$$
//! for posting them automatically.
//!
//! IOW it's self-hosted automatic tweet posting software..
//!
//! # Library doc
//!
//! This library is used by `not-stakkr` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! See documentation for `ops::*` submodules as each one has a distinct data flow.
//!
//! # Executable doc
//!
//! Exit values and possible errors:
//!
//! ```plaintext
//! 1 - a file would need to be overriden, but was not allowed to
//! 2 - required data or file needs to be created by running the specified filesysstem
//! 3 - an error was returned by the Twitter API
//! 4 - failed to parse the specified file
//! ```
//!
//! ## Executable manpage
//!
//! [All manpages](https://cdn.rawgit.com/nabijaczleweli/not-stakkr/man/index.html)


extern crate rustc_serialize;
extern crate egg_mode;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate toml;

mod outcome;

pub mod ops;
pub mod util;
pub mod options;

pub use outcome::Outcome;
