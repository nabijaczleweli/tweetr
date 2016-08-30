use std::io::{Read, Write, Error as IoError};
use toml::{encode_str, decode_str};
use egg_mode::Token;
use std::path::Path;
use std::fs::File;


/// The tokens needed to authenticate the app itself.
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct AppTokens {
    pub key: String,
    pub secret: String,
}

impl AppTokens {
    /// Read the application tokens from the specified file.
    pub fn read(p: &Path) -> Result<AppTokens, Option<IoError>> {
        let mut buf = String::new();
        try!(try!(File::open(p).map_err(Some)).read_to_string(&mut buf).map_err(Some));
        decode_str(&buf).ok_or(None)
    }

    /// Save the application tokens to the specified file.
    pub fn write(&self, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&self).as_bytes()).unwrap();
    }
}

impl<'a> Into<Token<'a>> for AppTokens {
    fn into(self) -> Token<'a> {
        Token::new(self.key, self.secret)
    }
}

impl<'a> From<Token<'a>> for AppTokens {
    fn from(tkn: Token<'a>) -> AppTokens {
        AppTokens {
            key: tkn.key.into_owned().to_string(),
            secret: tkn.secret.into_owned().to_string(),
        }
    }
}
