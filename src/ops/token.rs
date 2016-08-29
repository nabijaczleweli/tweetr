use toml::encode_str;
use std::path::Path;
use std::io::Write;
use std::fs::File;


#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable)]
pub struct AppTokens {
    pub key: String,
    pub secret: String,
}

impl AppTokens {
    pub fn write(&self, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&self).as_bytes()).unwrap();
    }
}
