//! Main functions doing actual work.
//!
//! Each module contains the functions for their respective subsystems.


use toml::{Parser, Value, decode};
use rustc_serialize::Decodable;
use std::path::{PathBuf, Path};
use self::super::Outcome;
use std::fs::File;
use std::io::Read;

mod user;
mod token;
mod queued_tweet;

pub mod init;
pub mod add_user;
pub mod start_daemon;

pub use self::user::User;
pub use self::token::AppTokens;
pub use self::queued_tweet::QueuedTweet;


fn verify_file(fname: &str, should_exist: bool, config_dir: &(String, PathBuf), force: bool, producing_subsystem: &'static str) -> Result<PathBuf, Outcome> {
    let app_data_file = config_dir.1.join(fname);

    if force || app_data_file.exists() == should_exist {
        Ok(app_data_file)
    } else {
        let filename = PathBuf::from(&config_dir.0).join(fname).to_str().unwrap().replace("\\", "/");

        if should_exist {
            Err(Outcome::RequiredFileFromSubsystemNonexistant {
                subsys: producing_subsystem,
                fname: filename,
            })
        } else {
            Err(Outcome::OverrideNoForce(filename))
        }
    }
}

fn read_toml_file<T: Decodable>(p: &Path, desc: &'static str) -> Result<T, Option<Outcome>> {
    let mut buf = String::new();
    try!(try!(File::open(p).map_err(|_| None)).read_to_string(&mut buf).map_err(|_| None));

    let mut parser = Parser::new(&buf);
    let parsed = parser.parse().and_then(|t| decode(Value::Table(t)));
    parsed.ok_or_else(move || {
        Some(Outcome::FileParsingFailed {
            desc: desc,
            errors: parser.errors
                .iter()
                .map(|e| {
                    let (line, col) = parser.to_linecol(e.lo);
                    format!("error: {}:{}: {}", line, col, e.desc)
                })
                .collect(),
        })
    })
}
