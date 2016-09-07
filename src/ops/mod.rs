//! Main functions doing actual work.
//!
//! Each module contains the functions for their respective subsystems.


use std::path::PathBuf;
use self::super::Outcome;

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
