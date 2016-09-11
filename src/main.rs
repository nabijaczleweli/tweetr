extern crate tweetr;

use std::thread;
use std::process::exit;
use std::path::PathBuf;
use std::time::Duration;
use std::io::{stdin, stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = tweetr::options::Options::parse();

    let err = match opts.subsystem.clone() {
            tweetr::options::Subsystem::Init { force } => init_main(opts, force),
            tweetr::options::Subsystem::AddUser { verbose } => add_user_main(opts, verbose),
            tweetr::options::Subsystem::QueueTweet { file_to_load } => queue_tweet_main(opts, file_to_load),
            tweetr::options::Subsystem::StartDaemon { delay, verbose } => start_daemon_main(opts, delay, verbose),
        }
        .err()
        .unwrap_or(tweetr::Outcome::NoError);
    err.print_error(&mut stderr());
    err.exit_value()
}

fn init_main(opts: tweetr::options::Options, force: bool) -> Result<(), tweetr::Outcome> {
    let app_path = try!(tweetr::ops::init::verify(&opts.config_dir, force));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let data = tweetr::ops::init::get_data(&mut lock, &mut stdout());
    data.write(&app_path);

    Ok(())
}

fn add_user_main(opts: tweetr::options::Options, verbose: bool) -> Result<(), tweetr::Outcome> {
    let (app_path, users_path) = try!(tweetr::ops::add_user::verify(&opts.config_dir));
    let app = try!(tweetr::ops::AppTokens::read(&app_path).map_err(Option::unwrap));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let user = try!(tweetr::ops::add_user::authorise(&mut lock, &mut stdout(), app, verbose));
    println!("");
    tweetr::ops::add_user::print_success_message(&mut stdout(), &user, verbose);

    Err(tweetr::ops::add_user::append_user(&users_path, user))
}

fn queue_tweet_main(opts: tweetr::options::Options, file_to_load: Option<PathBuf>) -> Result<(), tweetr::Outcome> {
    let tweets_path = tweetr::ops::queue_tweet::tweets_path(&opts.config_dir.1);

    let mut tweets_to_queue = match file_to_load {
        Some(ftl) => try!(tweetr::ops::QueuedTweet::read(&ftl).map_err(Option::unwrap)),
        None => {
            let stdin = stdin();
            let mut lock = stdin.lock();

            let mut ttq = Vec::new();
            while let Some(tweet) = tweetr::ops::queue_tweet::get_tweet(&mut lock, &mut stdout()) {
                ttq.push(tweet);
            }
            ttq
        }
    };

    let mut tweets = try!(tweetr::ops::QueuedTweet::read(&tweets_path).map_err(Option::unwrap));
    tweets.append(&mut tweets_to_queue);
    tweets.sort();

    tweetr::ops::QueuedTweet::write(tweets, &tweets_path);

    Ok(())
}

fn start_daemon_main(opts: tweetr::options::Options, delay: Duration, verbose: bool) -> Result<(), tweetr::Outcome> {
    let (app_path, users_path, tweets_path) = try!(tweetr::ops::start_daemon::verify(&opts.config_dir));
    let app = try!(tweetr::ops::AppTokens::read(&app_path).map_err(Option::unwrap));
    let app_tokens = app.raw_token();

    loop {
        match (tweetr::ops::User::read(&users_path), tweetr::ops::QueuedTweet::read(&tweets_path)) {
            (Ok(users), Ok(mut tweets)) => {
                let tweets_to_post = tweetr::ops::start_daemon::tweet_indices_to_post(&tweets);

                for i in tweets_to_post {
                    let tweet_to_post = &mut tweets[i];

                    match tweetr::ops::start_daemon::find_user_index_for_tweet(tweet_to_post, &users) {
                        Ok(user_i) => {
                            tweetr::ops::start_daemon::post_tweet(tweet_to_post, &users[user_i], &app_tokens, verbose, &mut stdout())
                                .print_error(&mut stderr());
                        }
                        Err(out) => out.print_error(&mut stderr()),
                    }
                }

                tweetr::ops::QueuedTweet::write(tweets, &tweets_path);

                thread::sleep(delay);
            }
            (Err(err), _) => err.unwrap().print_error(&mut stderr()),
            (_, Err(err)) => err.unwrap().print_error(&mut stderr()),
        }
    }
}
