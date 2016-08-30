//! Module containing various utility functions.


use std::io::{BufRead, Write, Result as IoResult, Error, ErrorKind};


/// Ask the user to input a string of the exact length of `desired_len`, (re)prompting as necessary.
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_exact_len;
/// assert_eq!(prompt_exact_len(&mut Cursor::new(b"0123456789"), &mut Vec::new(), "Allowed chars", 10).unwrap(),
///            "0123456789".to_string());
/// ```
pub fn prompt_exact_len<R: BufRead, W: Write>(input: &mut R, output: &mut W, prompt_s: &str, desired_len: usize) -> IoResult<String> {
    let mut out = String::new();

    while out.len() != desired_len {
        try!(prompt(input, output, prompt_s, &mut out));
    }

    Ok(out)
}

/// Ask the user to input a string of non-zero length, (re)prompting as necessary.
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// # use not_stakkr::util::prompt_exact_len;
/// assert_eq!(prompt_exact_len(&mut Cursor::new(b"0123456789"), &mut Vec::new(), "Allowed chars", 10).unwrap(),
///            "0123456789".to_string());
/// ```
pub fn prompt_nonzero_len<R: BufRead, W: Write>(input: &mut R, output: &mut W, prompt_s: &str) -> IoResult<String> {
    let mut out = String::new();

    while out.is_empty() {
        try!(prompt(input, output, prompt_s, &mut out));
    }

    Ok(out)
}


fn prompt<R: BufRead, W: Write>(input: &mut R, output: &mut W, prompt_s: &str, out: &mut String) -> IoResult<()> {
    try!(write!(output, "{}: ", prompt_s));
    try!(output.flush());

    out.clear();
    if try!(input.read_line(out)) == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Input too short"));
    }

    *out = out.trim().to_string();

    Ok(())
}
