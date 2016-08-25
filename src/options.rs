use clap::{self, App, Arg, AppSettings};
use std::path::PathBuf;
use std::env::home_dir;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory containing configuration. Default: `"$HOME/.not-stakkr`
    pub config_dir: PathBuf,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("checksums")
            .setting(AppSettings::ColoredHelp)
            .version(crate_version!())
            .author(crate_authors!())
            .about("not-stakkr is a platform that allows you to create and queue tweets to be shared when YOU want.\nYou create content when you have time \
                    and then use FOSS and NOT pay whatever-ridiculous amount of $$$ for posting them automatically")
            .arg(Arg::from_usage("-c --config-dir=[CONFIG_DIR] 'Directory containing configuration. Default: $HOME/.not-stakkr'")
                .validator(Options::config_dir_validator))
            .get_matches();

        Options {
            config_dir: match matches.value_of("config-dir") {
                Some(dirs) => fs::canonicalize(dirs).unwrap(),
                None => {
                    match home_dir() {
                        Some(mut hd) => {
                            hd = hd.canonicalize().unwrap();
                            hd.push(".not-stakkr");

                            fs::create_dir_all(&hd).unwrap();
                            hd
                        }
                        None => {
                            clap::Error {
                                    message: "Couldn't automatically get home directory, please specify configuration directory with the -c option".to_string(),
                                    kind: clap::ErrorKind::MissingRequiredArgument,
                                    info: None,
                                }
                                .exit()
                        }
                    }
                }
            },
        }
    }

    fn config_dir_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Configuration directory \"{}\" not found", s))
    }
}
