use self::super::super::util::{prompt_any_len, prompt_nonzero_len};
use self::super::{QueuedTweet, verify_file};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use chrono::DateTime;


pub fn tweets_path(config_dir: &(String, PathBuf)) -> PathBuf {
    verify_file("tweets.toml", true, config_dir, true, "").unwrap()
}

pub fn get_tweet<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> Option<QueuedTweet> {
    prompt_any_len(input, output, "Author (or empty to finish)", |_| true).unwrap().map(|uname| {
        let content = prompt_nonzero_len(input, output, "Tweet content", |_| true).unwrap();
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
