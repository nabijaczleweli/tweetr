extern crate not_stakkr;
extern crate chrono;

use self::not_stakkr::ops::QueuedTweet;
use self::chrono::{DateTime, Local};
use std::env::temp_dir;
use std::fs;


#[test]
fn empty_trans_eq() {
    trans_scaffold("empty_trans_eq", vec![]);
}

#[test]
fn unposted_trans_eq() {
    trans_scaffold("unposted_trans_eq", vec![unposted()]);
}

#[test]
fn posted_trans_eq() {
    trans_scaffold("posted_trans_eq", vec![posted()]);
}

#[test]
fn mixed_trans_eq() {
    trans_scaffold("mixed_trans_eq", vec![unposted(), posted()]);
}


fn trans_scaffold(name: &str, tweets: Vec<QueuedTweet>) {
    let td = temp_dir().join("not-stakkr-test").join(format!("ops-queued_tweet-{}", name));
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("tweets.toml");
    let _ = fs::remove_file(&tf);

    QueuedTweet::write(tweets.clone(), &tf);
    let read_tweets = QueuedTweet::read(&tf).unwrap();

    assert_eq!(tweets, read_tweets);
}

fn unposted() -> QueuedTweet {
    QueuedTweet {
        author: "nabijaczleweli".to_string(),
        time: DateTime::parse_from_rfc2822("Tue, 1 Jul 2098 10:52:37 +0200").unwrap(),
        content: "This tweet was not posted yet, so das good".to_string(),
        time_posted: None,
        id: None,
    }
}

fn posted() -> QueuedTweet {
    let now = Local::now();
    QueuedTweet {
        author: "nabijaczleweli".to_string(),
        time: DateTime::parse_from_rfc2822("Sat, 1 Jul 2000 15:12:57 -0800").unwrap(),
        content: "This tweet got posted just now, aww yeah, boii".to_string(),
        time_posted: Some(now.with_timezone(now.offset())),
        id: Some(420),
    }
}
