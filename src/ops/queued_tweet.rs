//! Queued tweets to be posted
//!
//! Okay, so here's how this shit works.
//!
//! We do not serialise `QueuedTweet` directly, because it does ugly things to serialise `DateTime` (namely: subkeys).
//! Instead, we convert it to a `QueuedTweetForSerialisation`, which has `DateTime`s converted to a `String` in RFC-3339
//! format and then serialise the `Vec` of them via `QueuedTweets` (classic trick).
//!
//! We do the inverse for deserialisation and silently return `None` if the `DateTime` strings have invalid format (same thing
//! we do with a `ParserError`, TODO: don't silent-ignore parser errors (applies to all serialisables))


use chrono::{DateTime, FixedOffset, ParseError};
use self::super::super::Outcome;
use self::super::read_toml_file;
use std::iter::FromIterator;
use std::cmp::Ordering;
use toml::encode_str;
use std::path::Path;
use std::io::Write;
use std::fs::File;


/// The struct representing a queued tweet to post, posted or not.
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct QueuedTweet {
    /// The author to post on behalf of.
    ///
    /// Has to have been previously added via the `add-user` subsystem.
    pub author: String,
    /// The time to post the tweet at.
    pub time: DateTime<FixedOffset>,

    /// The string content of the tweet.
    pub content: String,

    /// The time this tweet was posted.
    ///
    /// Becomes non-empty when posted.
    pub time_posted: Option<DateTime<FixedOffset>>,
    /// The numeric ID of the posted tweet.
    ///
    /// The tweet is accessible via the standard `https://twitter.com/{author}/{id}` URL.
    ///
    /// Becomes non-empty when posted.
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct QueuedTweetForSerialisation {
    pub author: String,
    pub time: String,

    pub content: String,

    pub time_posted: Option<String>,
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct QueuedTweets {
    tweet: Vec<QueuedTweetForSerialisation>,
}


impl QueuedTweet {
    /// Read all queued tweets from the specified file.
    pub fn read(p: &Path) -> Result<Vec<QueuedTweet>, Option<Outcome>> {
        let queued_tweets: QueuedTweets = try!(read_toml_file(p, "queued tweets"));
        Result::from_iter(queued_tweets.tweet.into_iter().map(|qts| qts.into()).collect::<Vec<_>>()).map_err(|_| None)
    }

    /// Save all queued tweets to the specified file.
    pub fn write(queued_tweets: Vec<QueuedTweet>, p: &Path) {
        File::create(p)
            .unwrap()
            .write_all(encode_str(&QueuedTweets { tweet: queued_tweets.into_iter().map(QueuedTweetForSerialisation::from).collect() }).as_bytes())
            .unwrap();
    }
}

impl Ord for QueuedTweet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for QueuedTweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}


impl From<QueuedTweet> for QueuedTweetForSerialisation {
    fn from(qt: QueuedTweet) -> QueuedTweetForSerialisation {
        QueuedTweetForSerialisation {
            author: qt.author,
            time: qt.time.to_rfc3339(),
            content: qt.content,
            time_posted: qt.time_posted.map(|dt| dt.to_rfc3339()),
            id: qt.id,
        }
    }
}

impl Into<Result<QueuedTweet, ParseError>> for QueuedTweetForSerialisation {
    fn into(self) -> Result<QueuedTweet, ParseError> {
        Ok(QueuedTweet {
            author: self.author,
            time: try!(DateTime::parse_from_rfc3339(&self.time)),
            content: self.content,
            time_posted: match self.time_posted {
                Some(dts) => Some(try!(DateTime::parse_from_rfc3339(&dts))),
                None => None,
            },
            id: self.id,
        })
    }
}
