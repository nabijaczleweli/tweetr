extern crate not_stakkr;

use self::not_stakkr::util::prompt_any_len;
use std::iter::FromIterator;
use std::io::Cursor;


#[test]
fn optimistic() {
    let mut out = Vec::new();
    let result = prompt_any_len(&mut Cursor::new(b"0!1\n"), &mut out, "Allowed chars", |s| s.contains('!')).unwrap();

    assert_eq!(result, Some("0!1".to_string()));
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}

#[test]
fn bad() {
    let mut out = Vec::new();
    let result = prompt_any_len(&mut Cursor::new(b"01\n"), &mut out, "Allowed chars", |s| s.contains('!')).unwrap();

    assert_eq!(result, None);
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}
