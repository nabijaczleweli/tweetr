use self::super::super::Outcome;
use self::super::read_toml_file;
use toml::encode_str;
use egg_mode::Token;
use std::path::Path;
use std::io::Write;
use std::fs::File;


/// The tokens needed to authenticate the app itself.
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct AppTokens {
    /// Key part of the authentication token
    pub key: String,
    /// Secret part of the authentication token
    pub secret: String,
}

impl AppTokens {
    /// Read the application tokens from the specified file.
    pub fn read(p: &Path) -> Result<AppTokens, Option<Outcome>> {
        read_toml_file(p, "application tokens")
    }

    /// Save the application tokens to the specified file.
    pub fn write(&self, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&self).as_bytes()).unwrap();
    }

    /// Borrows the current key and secret into an `egg_mode::Token`.
    pub fn raw_token<'a>(&'a self) -> Token<'a> {
        Token::new(&self.key[..], &self.secret[..])
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
