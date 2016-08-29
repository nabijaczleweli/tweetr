extern crate not_stakkr;

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
                    println!("{:?}", pb);
                    let stdin = stdin();
                    let mut lock = stdin.lock();

                    let data = not_stakkr::ops::init::get_data(&mut lock, &mut stdout());
                    data.write(&pb);
                    not_stakkr::Outcome::NoError
                }
                Err(out) => out,
            }
        }
        not_stakkr::options::Subsystem::AddUser => not_stakkr::Outcome::NoError,
    };
    err.print_error(&mut stderr());
    err.exit_value()
}
