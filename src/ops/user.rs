use self::super::super::Outcome;
use self::super::read_toml_file;
use std::cmp::Ordering;
use toml::encode_str;
use egg_mode::Token;
use std::path::Path;
use std::io::Write;
use std::fs::File;


/// All user data required to connect to the Twitter API.
#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    /// The user's name (not display name)
    pub name: String,
    /// User id
    pub id: i64,
    /// The key part of the access token (the one actually used to access the API)
    pub access_token_key: String,
    /// The key part of the access token (the one actually used to access the API)
    pub access_token_secret: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct Users {
    user: Vec<User>,
}


impl User {
    /// Create a `User` instance straight from the return value of `egg_mode::access_token()`
    pub fn from_raw_access_token<'t>(raw: (Token<'t>, i64, String)) -> User {
        let (access_token, user_id, username) = raw;
        User {
            name: username,
            id: user_id,
            access_token_key: access_token.key.to_owned().to_string(),
            access_token_secret: access_token.secret.to_owned().to_string(),
        }
    }

    /// Read all user data from the specified file.
    pub fn read(p: &Path) -> Result<Vec<User>, Option<Outcome>> {
        read_toml_file(p, "users").map(|us: Users| us.user)
    }

    /// Save all user data to the specified file.
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
