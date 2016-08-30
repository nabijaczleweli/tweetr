extern crate not_stakkr;

use self::not_stakkr::util::prompt_exact_len;
use std::iter::FromIterator;
use std::io::Cursor;


#[test]
fn optimistic() {
    let mut out = Vec::new();
    let result = prompt_exact_len(&mut Cursor::new(b"0!1\n"), &mut out, "Allowed chars", |s| s.contains('!'), 3).unwrap();

    assert_eq!(result, "0!1".to_string());
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}

#[test]
fn bad() {
    let mut out = Vec::new();
    prompt_exact_len(&mut Cursor::new(b"01\n"), &mut out, "Allowed chars", |s| s.contains('!'), 3).unwrap_err();
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}

#[test]
fn bad_but_then_ok() {
    let mut out = Vec::new();
    let result = prompt_exact_len(&mut Cursor::new(b"01\n0!1\n"), &mut out, "Allowed chars", |s| s.contains('!'), 3).unwrap();

    assert_eq!(result, "0!1".to_string());
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}
