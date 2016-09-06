//! Module containing various utility functions.


use std::io::{BufRead, Write, Result as IoResult, Error, ErrorKind};


/// The datetime format returned by Twitter when posting.
///
/// # Examples
///
/// ```
/// # extern crate not_stakkr;
/// # extern crate chrono;
/// # use not_stakkr::util::TWEET_DATETIME_FORMAT;
/// # use chrono::DateTime;
/// # fn main() {
/// assert_eq!(DateTime::parse_from_str("Mon Sep 05 20:30:51 +0000 2016", TWEET_DATETIME_FORMAT),
///            DateTime::parse_from_rfc3339("2016-09-05T20:30:51+00:00"));
/// # }
/// ```
pub static TWEET_DATETIME_FORMAT: &'static str = "%a %b %d %T %z %Y";


/// Ask the user to input a string of the exact length of `desired_len`, (re)prompting as necessary.
///
/// # Examples
///
/// Allow anything 10 charactes long:
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_exact_len;
/// assert_eq!(prompt_exact_len(&mut Cursor::new(b"0123456789"),
///                             &mut Vec::new(),
///                             "Allowed chars",
///                             |_| true,
///                             10).unwrap(),
///            "0123456789".to_string());
/// ```
///
/// Allow a 10-character-long `u64`:
///
/// ```
/// # use std::io::Cursor;
/// # use std::str::FromStr;
/// # use not_stakkr::util::prompt_exact_len;
/// assert_eq!(prompt_exact_len(&mut Cursor::new(b"1234567890"),
///                             &mut Vec::new(),
///                             "Long number",
///                             |s| u64::from_str(s).is_ok(),
///                             10).unwrap(),
///            "1234567890".to_string());
/// assert!(prompt_exact_len(&mut Cursor::new(b"1234abcdef"),
///                          &mut Vec::new(),
///                          "Long number",
///                          |s| u64::from_str(s).is_ok(),
///                          10).is_err());
/// ```
pub fn prompt_exact_len<R, W, F>(input: &mut R, output: &mut W, prompt_s: &str, verifier: F, desired_len: usize) -> IoResult<String>
    where R: BufRead,
          W: Write,
          F: Fn(&String) -> bool
{
    let mut out = String::new();

    while out.len() != desired_len {
        try!(prompt(input, output, prompt_s, &verifier, &mut out));
    }

    Ok(out)
}

/// Ask the user to input a string of non-zero length, (re)prompting as necessary.
///
/// # Examples
///
/// Allow anything as long as it's *some*thing:
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_nonzero_len;
/// assert_eq!(prompt_nonzero_len(&mut Cursor::new(b"123456789"),
///                               &mut Vec::new(),
///                               "Allowed chars",
///                               |_| true).unwrap(),
///            "123456789".to_string());
/// ```
///
/// Allow valid `u64`s:
///
/// ```
/// # use std::io::Cursor;
/// # use std::str::FromStr;
/// # use not_stakkr::util::prompt_nonzero_len;
/// assert_eq!(prompt_nonzero_len(&mut Cursor::new(b"123456789"),
///                               &mut Vec::new(),
///                               "Number",
///                               |s| u64::from_str(s).is_ok()).unwrap(),
///            "123456789".to_string());
/// assert!(prompt_nonzero_len(&mut Cursor::new(b"123abcdef"),
///                            &mut Vec::new(),
///                            "Number",
///                            |s| u64::from_str(s).is_ok()).is_err());
/// ```
pub fn prompt_nonzero_len<R, W, F>(input: &mut R, output: &mut W, prompt_s: &str, verifier: F) -> IoResult<String>
    where R: BufRead,
          W: Write,
          F: Fn(&String) -> bool
{
    let mut out = String::new();

    while out.is_empty() {
        try!(prompt(input, output, prompt_s, &verifier, &mut out));
    }

    Ok(out)
}


fn prompt<R, W, F>(input: &mut R, output: &mut W, prompt_s: &str, verifier: &F, out: &mut String) -> IoResult<()>
    where R: BufRead,
          W: Write,
          F: Fn(&String) -> bool
{
    try!(write!(output, "{}: ", prompt_s));
    try!(output.flush());

    out.clear();
    if try!(input.read_line(out)) == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Input too short"));
    }

    *out = out.trim().to_string();
    if !verifier(out) {
        out.clear();
    }

    Ok(())
}
