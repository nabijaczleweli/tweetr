//! Main functions doing actual work.
//!
//! Each module contains the functions for their respective subsystems.


use std::path::PathBuf;

mod user;
mod token;

pub mod init;
pub mod add_user;

pub use self::user::User;
pub use self::token::AppTokens;


fn verify_file(fname: &str, should_exist: bool, config_dir: &(String, PathBuf), force: bool) -> Result<PathBuf, String> {
    let app_data_file = config_dir.1.join(fname);

    if force || app_data_file.exists() == should_exist {
        Ok(app_data_file)
    } else {
        Err(PathBuf::from(&config_dir.0).join(fname).to_str().unwrap().replace("\\", "/"))
    }
}
