use egg_mode::{Token, request_token, authorize_url, access_token};
use self::super::super::util::prompt_exact_len;
use self::super::{User, verify_file};
use self::super::super::Outcome;
use std::path::{Path, PathBuf};
use std::io::{BufRead, Write};
use std::str::FromStr;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Outcome> {
    let app = try!(verify_file("app.toml", true, config_dir, false).map_err(|f| {
        Outcome::RequiredFileFromSubsystemNonexistant {
            subsys: "init",
            fname: f,
        }
    }));

    Ok((app, config_dir.1.join("users.toml")))
}

pub fn authorise<'t, R: BufRead, W: Write, T: Into<Token<'t>>>(input: &mut R, output: &mut W, conn_token: T) -> Result<User, Outcome> {
    let conn_token = conn_token.into();
    let req_token = try!(request_token(&conn_token, "oob").map_err(|e| Outcome::TwitterAPIError(format!("{}", e))));
    let url = authorize_url(&req_token);

    writeln!(output, "Visit this URL: {}", url).unwrap();

    let pin = prompt_exact_len(input, output, "Enter the PIN from that page", |s| u32::from_str(s).is_ok(), 7).unwrap();

    let access_token_data = try!(access_token(&conn_token, &req_token, pin).map_err(|e| Outcome::TwitterAPIError(format!("{}", e))));
    Ok(User::from_raw_access_token(access_token_data))
}

pub fn append_user(users_path: &Path, user: User) {
    let mut users = if users_path.exists() {
        User::read(users_path).unwrap()
    } else {
        vec![]
    };

    match users.binary_search_by(|u| u.cmp(&user)) {
        Ok(curidx) => users[curidx] = user,
        Err(possidx) => users.insert(possidx, user),
    }

    User::write(users, &users_path);
}
