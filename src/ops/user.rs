use std::io::{Read, Write, Error as IoError};
use toml::{encode_str, decode_str};
use std::cmp::Ordering;
use egg_mode::Token;
use std::path::Path;
use std::fs::File;


#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub id: i64,
    pub access_token_key: String,
    pub access_token_secret: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct Users {
    user: Vec<User>,
}


impl User {
    pub fn from_raw_access_token<'t>(raw: (Token<'t>, i64, String)) -> User {
        let (access_token, user_id, username) = raw;
        User {
            name: username,
            id: user_id,
            access_token_key: access_token.key.to_owned().to_string(),
            access_token_secret: access_token.secret.to_owned().to_string(),
        }
    }

    pub fn read(p: &Path) -> Result<Vec<User>, Option<IoError>> {
        let mut buf = String::new();
        try!(try!(File::open(p).map_err(Some)).read_to_string(&mut buf).map_err(Some));

        let users: Users = try!(decode_str(&buf).ok_or(None));
        Ok(users.user)
    }

    pub fn write(users: Vec<User>, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&Users { user: users }).as_bytes()).unwrap();
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
