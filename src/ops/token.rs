use toml::encode_str;
use std::path::Path;
use std::io::Write;
use std::fs::File;


/// The tokens needed to authenticate the app itself.
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable)]
pub struct AppTokens {
    pub key: String,
    pub secret: String,
}

impl AppTokens {
    /// Save the application tokens to the specified file.
    pub fn write(&self, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&self).as_bytes()).unwrap();
    }
}
