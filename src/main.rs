extern crate not_stakkr;

use std::thread;
use std::process::exit;
use std::time::Duration;
use std::io::{stdin, stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = not_stakkr::options::Options::parse();
    println!("{:#?}", opts);

    let err = match opts.subsystem {
            not_stakkr::options::Subsystem::Init { force } => init_main(opts, force),
            not_stakkr::options::Subsystem::AddUser { verbose } => add_user_main(opts, verbose),
            not_stakkr::options::Subsystem::QueueTweet => queue_tweet_main(opts),
            not_stakkr::options::Subsystem::StartDaemon { delay, verbose } => start_daemon_main(opts, delay, verbose),
        }
        .err()
        .unwrap_or(not_stakkr::Outcome::NoError);
    err.print_error(&mut stderr());
    err.exit_value()
}

fn init_main(opts: not_stakkr::options::Options, force: bool) -> Result<(), not_stakkr::Outcome> {
    let app_path = try!(not_stakkr::ops::init::verify(&opts.config_dir, force));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let data = not_stakkr::ops::init::get_data(&mut lock, &mut stdout());
    data.write(&app_path);

    Ok(())
}

fn add_user_main(opts: not_stakkr::options::Options, verbose: bool) -> Result<(), not_stakkr::Outcome> {
    let (app_path, users_path) = try!(not_stakkr::ops::add_user::verify(&opts.config_dir));
    let app = try!(not_stakkr::ops::AppTokens::read(&app_path).map_err(Option::unwrap));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let user = try!(not_stakkr::ops::add_user::authorise(&mut lock, &mut stdout(), app, verbose));
    println!("");
    not_stakkr::ops::add_user::print_success_message(&mut stdout(), &user, verbose);

    Err(not_stakkr::ops::add_user::append_user(&users_path, user))
}

fn queue_tweet_main(opts: not_stakkr::options::Options) -> Result<(), not_stakkr::Outcome> {
    let tweets_path = not_stakkr::ops::queue_tweet::tweets_path(&opts.config_dir.1);

    let stdin = stdin();
    let mut lock = stdin.lock();

    let mut tweets_to_queue = Vec::new();
    while let Some(tweet) = not_stakkr::ops::queue_tweet::get_tweet(&mut lock, &mut stdout()) {
        tweets_to_queue.push(tweet);
    }

    let mut tweets = try!(not_stakkr::ops::QueuedTweet::read(&tweets_path).map_err(Option::unwrap));
    tweets.append(&mut tweets_to_queue);
    tweets.sort();

    not_stakkr::ops::QueuedTweet::write(tweets, &tweets_path);

    Ok(())
}

fn start_daemon_main(opts: not_stakkr::options::Options, delay: Duration, verbose: bool) -> Result<(), not_stakkr::Outcome> {
    let (app_path, users_path, tweets_path) = try!(not_stakkr::ops::start_daemon::verify(&opts.config_dir));
    let app = try!(not_stakkr::ops::AppTokens::read(&app_path).map_err(Option::unwrap));
    let app_tokens = app.raw_token();

    loop {
        match (not_stakkr::ops::User::read(&users_path), not_stakkr::ops::QueuedTweet::read(&tweets_path)) {
            (Ok(users), Ok(mut tweets)) => {
                let tweets_to_post = not_stakkr::ops::start_daemon::tweet_indices_to_post(&tweets);

                for i in tweets_to_post {
                    let tweet_to_post = &mut tweets[i];

                    match not_stakkr::ops::start_daemon::find_user_index_for_tweet(tweet_to_post, &users) {
                        Ok(user_i) => {
                            not_stakkr::ops::start_daemon::post_tweet(tweet_to_post, &users[user_i], &app_tokens, verbose, &mut stdout())
                                .print_error(&mut stderr());
                        }
                        Err(out) => out.print_error(&mut stderr()),
                    }
                }

                not_stakkr::ops::QueuedTweet::write(tweets, &tweets_path);

                thread::sleep(delay);
            }
            (Err(err), _) => err.unwrap().print_error(&mut stderr()),
            (_, Err(err)) => err.unwrap().print_error(&mut stderr()),
        }
    }
}
