use std::io::{BufRead, Write, Result as IoResult, Error, ErrorKind};


/// Ask the user to input a string of the exact length of `desired_len`, (re)prompting as necessary.
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// # extern crate not_stakkr;
/// # use not_stakkr::util::prompt_exact_len;
/// # fn main() {
/// assert_eq!(prompt_exact_len(&mut Cursor::new(b"0123456789"), &mut Vec::new(), "Allowed chars", 10).unwrap(),
///            "0123456789".to_string());
/// # }
/// ```
pub fn prompt_exact_len<R: BufRead, W: Write>(input: &mut R, output: &mut W, prompt: &str, desired_len: usize) -> IoResult<String> {
    let mut out = String::new();

    while out.len() != desired_len {
        try!(write!(output, "{}: ", prompt));
        try!(output.flush());

        out.clear();
        if try!(input.read_line(&mut out)) == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Input too short"));
        }

        out = out.trim().to_string();
    }

    Ok(out)
}
