extern crate not_stakkr;
use self::not_stakkr::util::prompt_exact_len;
use std::iter::FromIterator;
use std::io::Cursor;


#[test]
fn optimistic() {
    let mut out = Vec::new();
    let result = prompt_exact_len(&mut Cursor::new(b"0123456789\n"), &mut out, "Allowed chars", 10).unwrap();

    assert_eq!(result, "0123456789".to_string());
    assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
}

#[test]
fn empty() {
    let mut out = Vec::new();
    prompt_exact_len(&mut Cursor::new(b"\n"), &mut out, "Allowed chars", 10).unwrap_err();
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}

#[test]
fn empty_but_then_ok() {
    let mut out = Vec::new();
    let result = prompt_exact_len(&mut Cursor::new(b"\n0123456789\n"), &mut out, "Allowed chars", 10).unwrap();

    assert_eq!(result, "0123456789".to_string());
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}

#[test]
fn too_short() {
    let mut out = Vec::new();
    prompt_exact_len(&mut Cursor::new(b"0\n"), &mut out, "Allowed chars", 10).unwrap_err();
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}

#[test]
fn too_short_but_then_ok() {
    let mut out = Vec::new();
    let result = prompt_exact_len(&mut Cursor::new(b"0\n0123456789\n"), &mut out, "Allowed chars", 10).unwrap();

    assert_eq!(result, "0123456789".to_string());
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}

#[test]
fn too_long() {
    let mut out = Vec::new();
    prompt_exact_len(&mut Cursor::new(b"0123456789abcdef\n"), &mut out, "Allowed chars", 10).unwrap_err();
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}

#[test]
fn too_long_but_then_ok() {
    let mut out = Vec::new();
    let result = prompt_exact_len(&mut Cursor::new(b"0123456789abcdef\n0123456789\n"), &mut out, "Allowed chars", 10).unwrap();

    assert_eq!(result, "0123456789".to_string());
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
}
