use self::super::Outcome;
use std::path::Path;


pub fn init(config_dir: &Path, force: bool) -> Outcome {
    let app_data_file = config_dir.join("app.toml");

    if !app_data_file.exists() || force {
        Outcome::NoError
    } else {
        Outcome::OverrideNoForce(app_data_file.to_str().unwrap().to_string())
    }
}
