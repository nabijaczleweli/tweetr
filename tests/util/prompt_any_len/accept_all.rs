extern crate tweetr;

use self::tweetr::util::prompt_any_len;
use std::iter::FromIterator;
use std::io::Cursor;


#[test]
fn non_empty() {
    let mut out = Vec::new();
    let result = prompt_any_len(&mut Cursor::new(b"0123456789\n"), &mut out, "Allowed chars", |_| true).unwrap();

    assert_eq!(result, Some("0123456789".to_string()));
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}

#[test]
fn empty_nl() {
    let mut out = Vec::new();
    let result = prompt_any_len(&mut Cursor::new(b"\n"), &mut out, "Allowed chars", |_| true).unwrap();

    assert_eq!(result, None);
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}

#[test]
fn empty_nonl() {
    let mut out = Vec::new();
    let result = prompt_any_len(&mut Cursor::new(b""), &mut out, "Allowed chars", |_| true).unwrap();

    assert_eq!(result, None);
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}
