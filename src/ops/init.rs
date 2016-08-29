use self::super::super::util::prompt_exact_len;
use self::super::super::Outcome;
use std::io::{BufRead, Write};
use self::super::AppTokens;
use std::path::PathBuf;


pub fn verify(config_dir: &(String, PathBuf), force: bool) -> Result<PathBuf, Outcome> {
    let app_data_file = config_dir.1.join("app.toml");

    if force || !app_data_file.exists() {
        Ok(app_data_file)
    } else {
        Err(Outcome::OverrideNoForce(PathBuf::from(&config_dir.0).join("app.toml").to_str().unwrap().replace("\\", "/")))
    }
}

pub fn get_data<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> AppTokens {
    AppTokens {
        key: prompt_exact_len(input, output, "App key", 25).unwrap(),
        secret: prompt_exact_len(input, output, "App secret", 50).unwrap(),
    }
}
