//! This module contains the functions used only by the `add-user` subsystem.
//!
//! The flow of the `add-user` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::add_user::verify()
//! |> ops::AppTokens::read()
//! |> ops::add_user::authorise()
//! |> ops::add_user::append_user()
//! |> ops::add_user::print_success_message()
//! ```

use egg_mode::{Token, request_token, authorize_url, access_token};
use self::super::super::util::prompt_exact_len;
use self::super::{User, verify_file};
use self::super::super::Outcome;
use std::path::{Path, PathBuf};
use std::io::{BufRead, Write};
use std::str::FromStr;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `add-user`
/// subsystem.
///
/// The return value contains either the path to the file containing the global app configuration and the path to the file
/// containing the global users data or why getting them failed.
///
/// # Examples
///
/// Verifying with existing global app configuration.
///
/// ```
/// # use not_stakkr::ops::add_user;
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// let tf = temp_dir().join("not-stakkr-doctest").join("ops-add-user-verify-0");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("app.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(add_user::verify(&("$TEMP/ops-add-user-verify-0".to_string(), tf.clone())),
///            Ok((tf.join("app.toml"), tf.join("users.toml"))));
/// ```
///
/// Verifying when the global app configuration doesn't exist.
///
/// ```
/// # use not_stakkr::ops::add_user;
/// # use not_stakkr::Outcome;
/// # use std::env::temp_dir;
/// let tf = temp_dir().join("not-stakkr-doctest").join("ops-add-user-verify-1");
/// assert_eq!(add_user::verify(&("$TEMP/ops-add-user-verify-1".to_string(), tf)),
///            Err(Outcome::RequiredFileFromSubsystemNonexistant {
///                subsys: "init",
///                fname: "$TEMP/ops-add-user-verify-1/app.toml".to_string(),
///            }));
/// ```
pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Outcome> {
    let app = try!(verify_file("app.toml", true, config_dir, false, "init"));
    Ok((app, config_dir.1.join("users.toml")))
}

/// Direct the user towards the authorisation URL and prompt it for the PIN.
///
/// Returns `Err()` if accessing the Twitter API failed for whatever reason.
///
/// # Examples
///
/// ```no_run
/// # use not_stakkr::ops::{add_user, AppTokens};
/// # use std::io::BufReader;
/// assert!(add_user::authorise(&mut BufReader::new(b"1234567\n" as &[u8]), &mut Vec::new(), AppTokens {
///     key: "GeVFiYk7q8DhUmgMXE0iODrFa".to_string(),
///     secret: "bH3VIvYEwwVmMXkTnXB8N3HEQf4ShOf2Z4e1dkaqSJNGorK2pe".to_string(),
/// }, false).is_ok());
/// ```
pub fn authorise<'t, R, W, T>(input: &mut R, output: &mut W, conn_token: T, verbose: bool) -> Result<User, Outcome>
    where R: BufRead,
          W: Write,
          T: Into<Token<'t>>
{
    let conn_token = conn_token.into();

    let req_token = try!(wrap_network_op_in_ellipsis_done(output,
                                                          || {
                                                              let req_token = request_token(&conn_token, "oob")
                                                                  .map_err(|e| Outcome::TwitterAPIError(format!("{}", e)));
                                                              (req_token.is_ok(), req_token)
                                                          },
                                                          "request token",
                                                          verbose,
                                                          false,
                                                          false));
    let url = wrap_network_op_in_ellipsis_done(output, || (true, authorize_url(&req_token)), "authorisation URL", verbose, false, true);

    writeln!(output, "Visit this URL: {}", url).unwrap();

    let pin = prompt_exact_len(input, output, "Enter the PIN from that page", |s| u32::from_str(s).is_ok(), 7).unwrap();

    let access_token_data = try!(wrap_network_op_in_ellipsis_done(output,
                                                                  || {
                                                                      let access_token_data = access_token(&conn_token, &req_token, pin)
                                                                          .map_err(|e| Outcome::TwitterAPIError(format!("{}", e)));
                                                                      (access_token_data.is_ok(), access_token_data)
                                                                  },
                                                                  "access token",
                                                                  verbose,
                                                                  true,
                                                                  false));
    Ok(User::from_raw_access_token(access_token_data))
}

/// Append the specified user to the authenticated users list at the specified path.
///
/// # Examples
///
/// ```
/// # use not_stakkr::ops::{add_user, User};
/// # use std::env::temp_dir;
/// # use std::fs;
/// let tf = temp_dir().join("not-stakkr-doctest").join("ops-add-user-append_users");
/// fs::create_dir_all(&tf).unwrap();
///
/// let tf = tf.join("users.toml");
/// add_user::append_user(&tf, User {
///     name: "random-test-name".to_string(),
///     id: 0x969696969,
///     access_token_key: "40423221609-Y0klmK9nWNRAScBuumWvAtSOzmIvBIBLJpc3Ept".to_string(),
///     access_token_secret: "zFYbEO5wQtST3eK84pGuzSmmEByZbQ0EVY8uAS4BCM1mx".to_string(),
/// });
/// assert!(tf.exists());
/// ```
pub fn append_user(users_path: &Path, user: User) -> Outcome {
    let mut users = if users_path.exists() {
        match User::read(users_path).map_err(Option::unwrap) {
            Ok(users) => users,
            Err(out) => return out,
        }
    } else {
        vec![]
    };

    match users.binary_search_by(|u| u.cmp(&user)) {
        Ok(curidx) => users[curidx] = user,
        Err(possidx) => users.insert(possidx, user),
    }

    User::write(users, &users_path);
    Outcome::NoError
}

/// Print the success message mentioning the specified user's name and ID, optionally also mentioning tokens.
///
/// # Examples
///
/// ```
/// # use not_stakkr::ops::{add_user, User};
/// # use std::iter::FromIterator;
/// let mut out = Vec::new();
/// add_user::print_success_message(&mut out, &User {
///     name: "random-test-name".to_string(),
///     id: 0x42069,
///     access_token_key: "270441-N48kdEQFWtj7cUyWomNeE2AsNQw8pnmOaQbcwnV".to_string(),
///     access_token_secret: "jCcBthGzve36QMt3RAV6jOEg4qtHt7laMV2YFA3qKCRzw".to_string(),
/// }, false);
/// assert_eq!(out, Vec::from_iter(b"Successfully authenticated user random-test-name#270441\n".iter().cloned()));
/// ```
pub fn print_success_message<W: Write>(output: &mut W, user: &User, verbose: bool) {
    writeln!(output, "Successfully authenticated user {}#{}", user.name, user.id).unwrap();
    if verbose {
        writeln!(output, "Access tokens:").unwrap();
        writeln!(output, "  Key   : {}", user.access_token_key).unwrap();
        writeln!(output, "  Secret: {}", user.access_token_secret).unwrap();
    }
}


fn wrap_network_op_in_ellipsis_done<W, T, F>(output: &mut W, f: F, desc: &str, verbose: bool, nl_before: bool, nl_after: bool) -> T
    where W: Write,
          F: FnOnce() -> (bool, T)
{
    if verbose {
        if nl_before {
            writeln!(output, "").unwrap();
        }
        write!(output, "Getting {}...", desc).unwrap();
        output.flush().unwrap();

        let (succeeded, res) = f();

        if succeeded {
            writeln!(output, " DONE").unwrap();
        } else {
            writeln!(output, " FAILED").unwrap();
        }
        if nl_after {
            writeln!(output, "").unwrap();
        }

        res
    } else {
        f().1
    }
}
