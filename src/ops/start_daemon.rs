//! This module contains the functions used only by the `start-daemon` subsystem.
//!
//! The flow of the `start-daemon` subsystem is as follows:
//!
//! Initialisation:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::start_daemon::verify()
//! |> ops::AppTokens::read()
//! ```
//!
//! Then, in a loop:
//!
//! ```plaintext
//! init_data
//! |> ops::User::read()
//! |> ops::QueuedTweet::read()
//! |> ops::start_daemon::tweet_indices_to_post()
//! |> ops::start_daemon::find_user_index_for_tweet()
//! |> ops::start_daemon::post_tweet()
//! ```


use self::super::super::util::TWEET_DATETIME_FORMAT;
use self::super::{QueuedTweet, User, verify_file};
use self::super::super::Outcome;
use egg_mode::tweet::DraftTweet;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use egg_mode::Token;
use std::io::Write;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `start-daemon`
/// subsystem.
///
/// The return value contains either the path to the file containing the global app configuration, the path to the file
/// containing the global users data and the path to the file containing the global queued tweets data or why getting them
/// failed.
///
/// # Examples
///
/// Verifying with everything existing.
///
/// ```
/// # use std::fs::{self, File};
/// # use not_stakkr::ops::start_daemon;
/// # use not_stakkr::Outcome;
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// let tf = temp_dir().join("not-stakkr-doctest").join("ops-start-daemon-verify-0");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("app.toml")).unwrap().write(&[]).unwrap();
/// File::create(tf.join("users.toml")).unwrap().write(&[]).unwrap();
/// File::create(tf.join("tweets.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(start_daemon::verify(&("$TEMP/ops-start-daemon-verify-0".to_string(), tf.clone())),
///            Ok((tf.join("app.toml"), tf.join("users.toml"), tf.join("tweets.toml"))));
/// ```
///
/// Verifying with users data nonexistant.
///
/// ```
/// # use std::fs::{self, File};
/// # use not_stakkr::ops::start_daemon;
/// # use not_stakkr::Outcome;
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// let tf = temp_dir().join("not-stakkr-doctest").join("ops-start-daemon-verify-1");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("app.toml")).unwrap().write(&[]).unwrap();
/// File::create(tf.join("tweets.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(start_daemon::verify(&("$TEMP/ops-start-daemon-verify-1".to_string(), tf)),
///            Err(Outcome::RequiredFileFromSubsystemNonexistant {
///                subsys: "add-user",
///                fname: "$TEMP/ops-start-daemon-verify-1/users.toml".to_string(),
///            }));
/// ```
pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf, PathBuf), Outcome> {
    let app = try!(verify_file("app.toml", true, config_dir, false, "init"));
    let users = try!(verify_file("users.toml", true, config_dir, false, "add-user"));
    let tweets = try!(verify_file("tweets.toml", true, config_dir, false, "queue-tweet"));

    Ok((app, users, tweets))
}

/// Get the indices of tweets to post now from the provided batch based on whether thy've been posted already and the current
/// time.
///
/// All returned indices are guaranteed to be valid.
///
/// # Examples
///
/// ```
/// # extern crate not_stakkr;
/// # extern crate chrono;
/// # use not_stakkr::ops::{QueuedTweet, start_daemon};
/// # use chrono::{Duration, Local};
/// # fn main() {
/// let now = Local::now();
/// let now = now.with_timezone(now.offset());
///
/// assert_eq!(start_daemon::tweet_indices_to_post(&vec![
///     QueuedTweet {
///         author: "nabijaczleweli".to_string(),
///         time: now + Duration::hours(1),
///         content: "This tweet is not going to be posted (it's too early)".to_string(),
///         time_posted: None,
///         id: None,
///     },
///     QueuedTweet {
///         author: "nabijaczleweli".to_string(),
///         time: now - Duration::hours(1),
///         content: "This tweet is going to be posted".to_string(),
///         time_posted: None,
///         id: None,
///     },
///     QueuedTweet {
///         author: "nabijaczleweli".to_string(),
///         time: now - Duration::hours(1),
///         content: "This tweet is not going to be posted (it already was)".to_string(),
///         time_posted: Some(now - Duration::minutes(30)),
///         id: Some(6908265),
///     },
/// ]), vec![1]);
/// # }
/// ```
pub fn tweet_indices_to_post(tweets: &Vec<QueuedTweet>) -> Vec<usize> {
    let now = Local::now();
    let now = now.with_timezone(now.offset());

    tweets.iter()
        .enumerate()
        .flat_map(|(i, ref t)| if t.id.is_none() && t.time <= now {
            Some(i)
        } else {
            None
        })
        .collect()
}

/// Try to get the index of the user to post the given tweet.
///
/// This will fail iff there's no suitable user.
///
/// The returned index guaranteed to be valid.
///
/// # Examples
///
/// Finding a non-existant user:
///
/// ```
/// # extern crate not_stakkr;
/// # extern crate chrono;
/// # use not_stakkr::ops::{QueuedTweet, User, start_daemon};
/// # use chrono::{Duration, Local};
/// # fn main() {
/// let now = Local::now();
/// let now = now.with_timezone(now.offset());
///
/// let tweet = QueuedTweet {
///     author: "nabijaczleweli".to_string(),
///     time: now,
///     content: "dummy".to_string(),
///     time_posted: None,
///     id: None,
/// };
///
/// assert!(start_daemon::find_user_index_for_tweet(&tweet, &vec![]).is_err());
/// assert!(start_daemon::find_user_index_for_tweet(&tweet, &vec![User {
///     name: "danerangLP".to_string(),
///     id: 0x4208142311,
///     access_token_key: "key".to_string(),
///     access_token_secret: "secret".to_string(),
/// }]).is_err());
/// # }
/// ```
///
/// Finding am existing user:
///
/// ```
/// # extern crate not_stakkr;
/// # extern crate chrono;
/// # use not_stakkr::ops::{QueuedTweet, User, start_daemon};
/// # use chrono::{Duration, Local};
/// # fn main() {
/// let now = Local::now();
/// let now = now.with_timezone(now.offset());
///
/// assert_eq!(start_daemon::find_user_index_for_tweet(&QueuedTweet {
///     author: "danerangLP".to_string(),
///     time: now,
///     content: "dummy".to_string(),
///     time_posted: None,
///     id: None,
/// }, &vec![User {
///     name: "danerangLP".to_string(),
///     id: 0x4208142311,
///     access_token_key: "key".to_string(),
///     access_token_secret: "secret".to_string(),
/// }]), Ok(0));
/// # }
/// ```
pub fn find_user_index_for_tweet(tweet: &QueuedTweet, users: &Vec<User>) -> Result<usize, Outcome> {
    match users.iter().enumerate().find(|&iu| iu.1.name == tweet.author).map(|iu| iu.0) {
        Some(uid) => Ok(uid),
        None => {
            Err(Outcome::RequiredDataFromSubsystemNonexistant {
                subsys: "add-user",
                desc: format!("add and authorise user with name \"{}\" (required for tweet \"{}\" scheduled for {:?})",
                              tweet.author,
                              tweet.content,
                              tweet.time),
            })
        }
    }
}

/// Post the specified tweet on behalf of the specified user and application, optionally printing progress.
///
/// The tweet is updated with the data returned by the Twitter API.
///
/// # Examples
///
/// ```no_run
/// # extern crate not_stakkr;
/// # extern crate chrono;
/// # use not_stakkr::ops::{QueuedTweet, AppTokens, User, start_daemon};
/// # use chrono::{Duration, Local};
/// # fn main() {
/// let now = Local::now();
/// let now = now.with_timezone(now.offset());
///
/// let mut tweet = QueuedTweet {
///     author: "nabijaczleweli".to_string(),
///     time: now,
///     content: "This tweet will be posted, no matter the cost!".to_string(),
///     time_posted: None,
///     id: None,
/// };
///
/// let result = start_daemon::post_tweet(&mut tweet, &User {
///     name: "nabijaczleweli".to_string(),
///     id: 0x81423,
///     access_token_key: "529443-FNlJkpZCE7a4Bbd7f1k65GtgaH7SmHlReWSESD4".to_string(),
///     access_token_secret: "GVQDq88qLtJ45KR6u44A6AljW31JSSippjdipQg6gPYE5".to_string(),
/// }, &AppTokens {
///     key: "qzuqpwr101q4RtK9mDorI9ndm".to_string(),
///     secret: "HW4YG3Kdcap5ovcZ5fZfBJFedKR6GQe9MtZDS9Gm34hXiirkU5".to_string(),
/// }.into(), false, &mut vec![]);
///
/// assert_eq!(result.exit_value(), 0);
/// assert!(tweet.time_posted.is_some());
/// assert!(tweet.id.is_some());
/// # }
/// ```
pub fn post_tweet<'a, W: Write>(tweet: &mut QueuedTweet, on_behalf_of: &User, app: &Token<'a>, verbose: bool, output: &mut W) -> Outcome {
    if verbose {
        write!(output, "Posting tweet scheduled for {:?}...", tweet.time).unwrap();
        output.flush().unwrap();
    }
    match DraftTweet::new(&tweet.content).send(app, &Token::new(&on_behalf_of.access_token_key[..], &on_behalf_of.access_token_secret[..])) {
        Ok(resp) => {
            if verbose {
                writeln!(output, " SUCCESS").unwrap();
            }

            tweet.time_posted = Some(DateTime::parse_from_str(&resp.response.created_at, TWEET_DATETIME_FORMAT).unwrap());
            tweet.id = Some(resp.response.id);

            writeln!(output,
                     "Posted tweet \"{}\" scheduled for {:?} by {} at {:?} with ID {}",
                     tweet.content,
                     tweet.time,
                     tweet.author,
                     tweet.time_posted.as_ref().unwrap(),
                     resp.response.id)
                .unwrap();

            Outcome::NoError
        }
        Err(e) => {
            if verbose {
                writeln!(output, " FAILED").unwrap();
            }
            Outcome::TwitterAPIError(e.to_string())
        }
    }
}
