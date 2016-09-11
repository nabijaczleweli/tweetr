//! Module containing various utility functions.


use std::io::{BufRead, Write, Result as IoResult, Error, ErrorKind};
use std::iter;


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


/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// # use not_stakkr::util::mul_str;
/// assert_eq!(mul_str("Го! ", 3), "Го! Го! Го! ".to_string());
/// ```
pub fn mul_str(what: &str, n: usize) -> String {
    iter::repeat(what).take(n).collect()
}

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
        try!(prompt(input, output, prompt_s, &verifier, false, true, &mut out));
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
        try!(prompt(input, output, prompt_s, &verifier, false, true, &mut out));
    }

    Ok(out)
}

/// Ask the user to input a string of any length after printing a prompt prompting.
///
/// Will return `None` if the string is empty.
///
/// # Examples
///
/// Allow anything:
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_any_len;
/// assert_eq!(prompt_any_len(&mut Cursor::new(b"123456789"),
///                           &mut Vec::new(),
///                           "Allowed chars",
///                           |_| true).unwrap().unwrap(),
///            "123456789".to_string());
/// assert_eq!(prompt_any_len(&mut Cursor::new(b""),
///                           &mut Vec::new(),
///                           "Allowed chars",
///                           |_| true).unwrap(),
///            None);
/// ```
///
/// Allow valid `u64`s:
///
/// ```
/// # use std::io::Cursor;
/// # use std::str::FromStr;
/// # use not_stakkr::util::prompt_any_len;
/// assert_eq!(prompt_any_len(&mut Cursor::new(b"123456789"),
///                           &mut Vec::new(),
///                           "Number",
///                           |s| u64::from_str(s).is_ok()).unwrap().unwrap(),
///            "123456789".to_string());
/// assert_eq!(prompt_any_len(&mut Cursor::new(b"123abcdef"),
///                           &mut Vec::new(),
///                           "Number",
///                           |s| u64::from_str(s).is_ok()).map_err(|_| ()),
///            Ok(None));
/// ```
pub fn prompt_any_len<R, W, F>(input: &mut R, output: &mut W, prompt_s: &str, verifier: F) -> IoResult<Option<String>>
    where R: BufRead,
          W: Write,
          F: Fn(&String) -> bool
{
    let mut out = String::new();
    try!(prompt(input, output, prompt_s, &verifier, true, true, &mut out));

    if out.is_empty() {
        Ok(None)
    } else {
        Ok(Some(out))
    }
}

/// Ask the user to input a multiline string, (re)prompting as necessary.
///
/// Each line is separated by a `\`, but can be escaped by `\\`, e.g.
///
/// ```plaintext
/// Prompt: Abolish\
///         the\
///         burgeoisie!
/// ```
///
/// Will yield `"Abolish\nthe\nburgeoisie!"`, but
///
/// ```plaintext
/// Prompt: Capitalism\\
/// ```
///
/// Will yield `r"Capitalism\"`.
///
/// # Examples
///
/// Reading multiple lines:
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_multiline;
/// assert_eq!(prompt_multiline(&mut Cursor::new(b"Line 1\\\nLine 2\\\nLine 3"),
///                             &mut Vec::new(),
///                             "Lines",
///                             |_| true).unwrap(),
///            "Line 1\nLine 2\nLine 3".to_string());
/// ```
///
/// Escaping newline:
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_multiline;
/// assert_eq!(prompt_multiline(&mut Cursor::new(b"Line 0\\\\\n"),
///                             &mut Vec::new(),
///                             "Escaped line",
///                             |_| true).unwrap(),
///            "Line 0\\".to_string());
/// ```
///
/// Accepting only two-line strings:
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_multiline;
/// assert_eq!(prompt_multiline(&mut Cursor::new(b"Line 1\\\nLine 2\n"),
///                             &mut Vec::new(),
///                             "2 lines",
///                             |s| s.lines().count() == 2).unwrap(),
///            "Line 1\nLine 2".to_string());
/// ```
pub fn prompt_multiline<R, W, F>(input: &mut R, output: &mut W, prompt_s: &str, verifier: F) -> IoResult<String>
    where R: BufRead,
          W: Write,
          F: Fn(&String) -> bool
{
    let reprompt = mul_str(" ", prompt_s.len() + 2);
    let mut lbuf = String::new();
    let mut buf = String::new();

    while buf.is_empty() {
        buf = try!(prompt_nonzero_len(input, output, prompt_s, |_| true));

        while buf.ends_with(r"\") && !buf.ends_with(r"\\") {
            buf.pop();
            buf.push('\n');

            try!(prompt(input, output, &reprompt, &|_| true, false, false, &mut lbuf));
            buf.push_str(&lbuf);
        }

        if buf.ends_with(r"\\") {
            buf.pop();
        }

        if !verifier(&buf) {
            buf.clear();
        }
    }

    Ok(buf)
}

fn prompt<R, W, F>(input: &mut R, output: &mut W, prompt_s: &str, verifier: &F, allow_empty: bool, colon: bool, out: &mut String) -> IoResult<()>
    where R: BufRead,
          W: Write,
          F: Fn(&String) -> bool
{
    if colon {
        try!(write!(output, "{}: ", prompt_s));
    } else {
        try!(write!(output, "{}", prompt_s));
    }
    try!(output.flush());

    out.clear();
    if try!(input.read_line(out)) == 0 && !allow_empty {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Input too short"));
    }

    *out = out.trim().to_string();
    if !verifier(out) {
        out.clear();
    }

    Ok(())
}
