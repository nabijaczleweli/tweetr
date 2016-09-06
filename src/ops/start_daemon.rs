use self::super::super::util::TWEET_DATETIME_FORMAT;
use self::super::{QueuedTweet, User, verify_file};
use self::super::super::Outcome;
use egg_mode::tweet::DraftTweet;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use egg_mode::Token;
use std::io::Write;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf, PathBuf), Outcome> {
    let app = try!(verify_file("app.toml", true, config_dir, false).map_err(|f| {
        Outcome::RequiredFileFromSubsystemNonexistant {
            subsys: "init",
            fname: f,
        }
    }));
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

    Ok((app, users, tweets))
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

pub fn post_tweet<'a, W: Write>(tweet: &mut QueuedTweet, on_behalf_of: &User, app: &Token<'a>, verbose: bool, output: &mut W) -> Outcome {
    if verbose {
        write!(output,
               "Posting tweet \"{}\" scheduled for {:?} by {}...",
               tweet.content,
               tweet.time,
               tweet.author)
            .unwrap();
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
