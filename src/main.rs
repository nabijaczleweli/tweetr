extern crate not_stakkr;

use std::thread;
use std::process::exit;
use std::io::{stdin, stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = not_stakkr::options::Options::parse();
    println!("{:#?}", opts);

    let err = match opts.subsystem {
        not_stakkr::options::Subsystem::Init { force } => {
            match not_stakkr::ops::init::verify(&opts.config_dir, force) {
                Ok(app_path) => {
                    let stdin = stdin();
                    let mut lock = stdin.lock();

                    let data = not_stakkr::ops::init::get_data(&mut lock, &mut stdout());
                    data.write(&app_path);
                    not_stakkr::Outcome::NoError
                }
                Err(out) => out,
            }
        }
        not_stakkr::options::Subsystem::AddUser { verbose } => {
            match not_stakkr::ops::add_user::verify(&opts.config_dir) {
                Ok((app_path, users_path)) => {
                    match not_stakkr::ops::AppTokens::read(&app_path).map_err(Option::unwrap) {
                        Ok(app) => {
                            let stdin = stdin();
                            let mut lock = stdin.lock();

                            match not_stakkr::ops::add_user::authorise(&mut lock, &mut stdout(), app, verbose) {
                                Ok(user) => {
                                    println!("");
                                    not_stakkr::ops::add_user::print_success_message(&mut stdout(), &user, verbose);

                                    not_stakkr::ops::add_user::append_user(&users_path, user)
                                }
                                Err(out) => out,
                            }
                        }
                        Err(out) => out,
                    }
                }
                Err(out) => out,
            }
        }
        not_stakkr::options::Subsystem::StartDaemon { delay, verbose } => {
            match not_stakkr::ops::start_daemon::verify(&opts.config_dir) {
                Ok((app_path, users_path, tweets_path)) => {
                    match not_stakkr::ops::AppTokens::read(&app_path).map_err(Option::unwrap) {
                        Ok(app) => {
                            let app_tokens = app.raw_token();

                            loop {
                                match (not_stakkr::ops::User::read(&users_path), not_stakkr::ops::QueuedTweet::read(&tweets_path)) {
                                    (Ok(users), Ok(mut tweets)) => {
                                        let tweets_to_post = not_stakkr::ops::start_daemon::tweet_indices_to_post(&tweets);

                                        for i in tweets_to_post {
                                            let tweet_to_post = &mut tweets[i];

                                            match not_stakkr::ops::start_daemon::find_user_index_for_tweet(tweet_to_post, &users) {
                                                Ok(user_i) => {
                                                    not_stakkr::ops::start_daemon::post_tweet(tweet_to_post,
                                                                                              &users[user_i],
                                                                                              &app_tokens,
                                                                                              verbose,
                                                                                              &mut stdout())
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
                        Err(out) => out,
                    }
                }
                Err(out) => out,
            }
        }
    };
    err.print_error(&mut stderr());
    err.exit_value()
}
