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
                Ok(pb) => {
                    let stdin = stdin();
                    let mut lock = stdin.lock();

                    let data = not_stakkr::ops::init::get_data(&mut lock, &mut stdout());
                    data.write(&pb);
                    not_stakkr::Outcome::NoError
                }
                Err(out) => out,
            }
        }
        not_stakkr::options::Subsystem::AddUser { verbose } => {
            match not_stakkr::ops::add_user::verify(&opts.config_dir) {
                Ok((app_path, users_path)) => {
                    let stdin = stdin();
                    let mut lock = stdin.lock();

                    let app = not_stakkr::ops::AppTokens::read(&app_path).unwrap();

                    match not_stakkr::ops::add_user::authorise(&mut lock, &mut stdout(), app, verbose) {
                        Ok(user) => {
                            println!("");
                            not_stakkr::ops::add_user::print_success_message(&mut stdout(), &user, verbose);

                            not_stakkr::ops::add_user::append_user(&users_path, user);

                            not_stakkr::Outcome::NoError
                        }
                        Err(out) => out,
                    }
                }
                Err(out) => out,
            }
        }
        not_stakkr::options::Subsystem::StartDaemon { delay, verbose } => {
            match not_stakkr::ops::start_daemon::verify(&opts.config_dir) {
                Ok((users_path, tweets_path)) => {
                    let users = not_stakkr::ops::User::read(&users_path).unwrap();

                    let result = None;
                    while result.is_none() {
                        let mut tweets = not_stakkr::ops::QueuedTweet::read(&tweets_path).unwrap();
                        let tweets_to_post = not_stakkr::ops::start_daemon::tweet_indices_to_post(&tweets);

                        for i in tweets_to_post {
                            let tweet_to_post = &mut tweets[i];

                            match not_stakkr::ops::start_daemon::find_user_index_for_tweet(tweet_to_post, &users) {
                                Ok(user_i) => {
                                    not_stakkr::ops::start_daemon::post_tweet(tweet_to_post, &users[user_i], verbose, &mut stdout()).print_error(&mut stderr());
                                }
                                Err(out) => out.print_error(&mut stderr()),
                            }
                        }

                        thread::sleep(delay);
                    }

                    result.unwrap()
                }
                Err(out) => out,
            }
        }
    };
    err.print_error(&mut stderr());
    err.exit_value()
}
