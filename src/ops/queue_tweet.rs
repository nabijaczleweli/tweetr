//! This module contains the functions used only by the `queue-tweet` subsystem.
//!
//! The flow of the `queue-tweet` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::queue_tweet::tweets_path()
//! |> ops::queue_tweet::get_tweet()
//! |> ops::QueuedTweet::read()
//! |> ops::QueuedTweet::write()
//! ```

use self::super::super::util::{prompt_any_len, prompt_nonzero_len, prompt_multiline};
use std::path::{PathBuf, Path};
use std::io::{BufRead, Write};
use self::super::QueuedTweet;
use chrono::DateTime;


/// Get the path to the file containing the global tweet queue.
///
/// # Examples
///
/// ```
/// # use not_stakkr::ops::queue_tweet;
/// # use std::env::temp_dir;
/// let tf = temp_dir().join("not-stakkr-doctest").join("ops-queue_tweets-tweets_path-0");
/// assert_eq!(queue_tweet::tweets_path(&tf), tf.join("tweets.toml"));
/// ```
pub fn tweets_path(config_dir: &Path) -> PathBuf {
    config_dir.join("tweets.toml")
}

/// Prompt the user for application data.
///
/// # Examples
///
/// Queueing a tweet.
///
/// ```
/// # extern crate not_stakkr;
/// # extern crate chrono;
/// # use not_stakkr::ops::{queue_tweet, QueuedTweet};
/// # use std::io::BufReader;
/// # use chrono::DateTime;
/// # fn main() {
/// assert_eq!(queue_tweet::get_tweet(&mut BufReader::new(b"not_stakkr_test\n\
///                                   Test tweet\n\
///                                   2016-09-09T00:33:30+02:00\n" as &[u8]),
///                                   &mut Vec::new()),
///            Some(QueuedTweet {
///                author: "not_stakkr_test".to_string(),
///                time: DateTime::parse_from_rfc3339("2016-09-09T00:33:30+02:00").unwrap(),
///                content: "Test tweet".to_string(),
///                time_posted: None,
///                id: None,
///            }));
/// # }
/// ```
///
/// Not queueing a tweet.
///
/// ```
/// # use not_stakkr::ops::{queue_tweet, QueuedTweet};
/// # use std::io::BufReader;
/// assert_eq!(queue_tweet::get_tweet(&mut BufReader::new(b"\n" as &[u8]), &mut Vec::new()), None);
/// ```
pub fn get_tweet<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> Option<QueuedTweet> {
    prompt_any_len(input, output, "Author (or empty to finish)", |_| true).unwrap().map(|uname| {
        let content = prompt_multiline(input, output, "Tweet content", |s| !s.trim().is_empty()).unwrap();
        let time = prompt_nonzero_len(input,
                                      output,
                                      "Time to post the tweet (RFC2822 or RFC3339)",
                                      |s| DateTime::parse_from_rfc2822(s).is_ok() || DateTime::parse_from_rfc3339(s).is_ok())
            .unwrap();

        writeln!(output, "").unwrap();
        QueuedTweet {
            author: uname,
            time: DateTime::parse_from_rfc2822(&time).or_else(|_| DateTime::parse_from_rfc3339(&time)).unwrap(),
            content: content,
            time_posted: None,
            id: None,
        }
    })
}
