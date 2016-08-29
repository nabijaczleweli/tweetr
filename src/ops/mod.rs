use self::super::Error;
use std::path::Path;


pub fn init(config_dir: &Path, force: bool) -> Error {
    let app_data_file = config_dir.join("app.toml");

    if !app_data_file.exists() || force {
        Error::NoError
    } else {
        Error::OverrideNoForce(app_data_file.to_str().unwrap().to_string())
    }
}
