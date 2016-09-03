use self::super::{QueuedTweet, User, verify_file};
use self::super::super::Outcome;
use std::path::PathBuf;
use std::io::Write;
use chrono::Local;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Outcome> {
    let users = try!(verify_file("users.toml", true, config_dir, false).map_err(|f| {
        Outcome::RequiredFileFromSubsystemNonexistant {
            subsys: "add-user",
            fname: f,
        }
    }));
    let tweets = try!(verify_file("tweets.toml", true, config_dir, false).map_err(|f| {
        Outcome::RequiredFileFromSubsystemNonexistant {
            subsys: "queue-tweet",
            fname: f,
        }
    }));

    Ok((users, tweets))
}

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

pub fn find_user_index_for_tweet<W: Write>(tweet: &QueuedTweet, users: &Vec<User>, err: &mut W) -> Option<usize> {
    let res = users.iter().enumerate().find(|&iu| iu.1.name == tweet.author).map(|iu| iu.0);

    if res.is_none() {
        writeln!(err,
                 "Couldn't find user with name \"{}\" for tweet \"{}\" scheduled for {:?}",
                 tweet.author,
                 tweet.content,
                 tweet.time)
            .unwrap();
    }

    res
}

// TODO: remove
#[allow(unused)]
pub fn post_tweet<W: Write>(tweet: &mut QueuedTweet, on_behalf_of: &User, verbose: bool, output: &mut W) -> Outcome {
    let now = Local::now();
    let now = now.with_timezone(now.offset());

    Outcome::TwitterAPIError(format!("Unimplemented (see #1): posting tweet \"{}\" on behalf of {} scheduled for {:?} failed",
                                     tweet.content,
                                     tweet.author,
                                     tweet.time))
}
