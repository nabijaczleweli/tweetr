use self::super::super::util::prompt_exact_len;
use self::super::super::Outcome;
use self::super::AppTokens;
use std::path::{PathBuf, Path};
use std::io::{BufRead, Write};


pub fn verify(config_dir: &Path, force: bool) -> Result<PathBuf, Outcome> {
    let app_data_file = config_dir.join("app.toml");

    if force || !app_data_file.exists() {
        Ok(app_data_file)
    } else {
        Err(Outcome::OverrideNoForce(app_data_file.to_str().unwrap().to_string()))
    }
}

pub fn get_data<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> AppTokens {
    AppTokens {
        key: prompt_exact_len(input, output, "App key", 25).unwrap(),
        secret: prompt_exact_len(input, output, "App secret", 50).unwrap(),
    }
}
