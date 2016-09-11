//! This module contains the functions used only by the `init` subsystem.
//!
//! The flow of the `init` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::init::verify()
//! |> ops::init::get_data()
//! |> ops::AppTokens::write()
//! ```


use self::super::super::util::prompt_exact_len;
use self::super::{AppTokens, verify_file};
use self::super::super::Outcome;
use std::io::{BufRead, Write};
use std::path::PathBuf;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `init` subsystem.
///
/// The return value contains either the path to the file containing the global app configuration or why getting it failed.
///
/// # Examples
///
/// Verifying a nonexistant file or an existing file with forcing.
///
/// ```
/// # use std::env::temp_dir;
/// # use tweetr::ops::init;
/// let tf = temp_dir().join("tweetr-doctest").join("ops-init-verify-0");
/// assert_eq!(init::verify(&("$TEMP/ops-init-verify-0".to_string(), tf.clone()), true),
///            Ok(tf.join("app.toml")));
/// ```
///
/// Verifying an existing file without forcing.
///
/// ```
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use tweetr::ops::init;
/// # use tweetr::Outcome;
/// # use std::io::Write;
/// let tf = temp_dir().join("tweetr-doctest").join("ops-init-verify-1");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("app.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(init::verify(&("$TEMP/ops-init-verify-1".to_string(), tf), false),
///            Err(Outcome::OverrideNoForce("$TEMP/ops-init-verify-1/app.toml".to_string())));
/// ```
pub fn verify(config_dir: &(String, PathBuf), force: bool) -> Result<PathBuf, Outcome> {
    verify_file("app.toml", false, config_dir, force, "")
}

/// Prompt the user for application data.
///
/// # Examples
///
/// ```
/// # use tweetr::ops::{init, AppTokens};
/// # use std::io::BufReader;
/// assert_eq!(init::get_data(
///                 &mut BufReader::new(b"qdPD7N8CcPYDKiNv81QWNWaHK\n\
///                                       U9A5CM1LzwNliBiHGPIJyx6tFYAGVr3bCMbVkWKu8Zb13kHD4p\n" as &[u8]),
///                 &mut Vec::new()),
///            AppTokens {
///                key: "qdPD7N8CcPYDKiNv81QWNWaHK".to_string(),
///                secret: "U9A5CM1LzwNliBiHGPIJyx6tFYAGVr3bCMbVkWKu8Zb13kHD4p".to_string(),
///            });
/// ```
pub fn get_data<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> AppTokens {
    AppTokens {
        key: prompt_exact_len(input, output, "App key", |_| true, 25).unwrap(),
        secret: prompt_exact_len(input, output, "App secret", |_| true, 50).unwrap(),
    }
}
