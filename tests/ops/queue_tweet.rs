mod get_tweet {
    extern crate tweetr;
    extern crate chrono;

    use self::tweetr::ops::{QueuedTweet, queue_tweet};
    use self::chrono::DateTime;
    use std::io::BufReader;


    #[test]
    fn rfc2822() {
        assert_eq!(queue_tweet::get_tweet(&mut BufReader::new(b"tweetr_test\n\
                                                                Test tweet\n\
                                                                Fri, 9 Sep 2016 00:33:30 +0200\n" as &[u8]),
                                          &mut Vec::new()),
                   Some(QueuedTweet {
                       author: "tweetr_test".to_string(),
                       time: DateTime::parse_from_rfc2822("Fri, 9 Sep 2016 00:33:30 +0200").unwrap(),
                       content: "Test tweet".to_string(),
                       time_posted: None,
                       id: None,
                   }));
    }

    #[test]
    fn empty() {
        assert_eq!(queue_tweet::get_tweet(&mut BufReader::new(b"" as &[u8]), &mut Vec::new()), None);
    }
}
