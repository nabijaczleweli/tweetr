mod single_line {
    extern crate not_stakkr;

    use self::not_stakkr::util::prompt_multiline;
    use std::iter::FromIterator;
    use std::io::Cursor;


    #[test]
    fn optimistic() {
        let mut out = Vec::new();
        let result = prompt_multiline(&mut Cursor::new(b"0123456789\n"), &mut out, "Allowed chars", |_| true).unwrap();

        assert_eq!(result, "0123456789".to_string());
        assert_eq!(out, Vec::from_iter(b"Allowed chars: ".iter().cloned()));
    }

    #[test]
    fn empty() {
        let mut out = Vec::new();
        prompt_multiline(&mut Cursor::new(b"\n"), &mut out, "Allowed chars", |_| true).unwrap_err();
        assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
    }

    #[test]
    fn empty_but_then_ok() {
        let mut out = Vec::new();
        let result = prompt_multiline(&mut Cursor::new(b"\n0123456789\n"), &mut out, "Allowed chars", |_| true).unwrap();

        assert_eq!(result, "0123456789".to_string());
        assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Allowed chars: Allowed chars: ".to_string());
    }
}

mod multi_line {
    extern crate not_stakkr;

    use self::not_stakkr::util::prompt_multiline;
    use std::iter::FromIterator;
    use std::io::Cursor;


    #[test]
    fn optimistic() {
        let mut out = Vec::new();
        let result = prompt_multiline(&mut Cursor::new(b"0123456789\\\n9876543210\n"), &mut out, "Allowed chars", |_| true).unwrap();

        assert_eq!(result, "0123456789\n9876543210".to_string());
        assert_eq!(out, Vec::from_iter(b"Allowed chars:                ".iter().cloned()));
    }

    #[test]
    fn empty_but_then_ok() {
        let mut out = Vec::new();
        let result = prompt_multiline(&mut Cursor::new(b"\n0123456789\\\n9876543210\n"), &mut out, "Allowed chars", |_| true).unwrap();

        assert_eq!(result, "0123456789\n9876543210".to_string());
        assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
                   "Allowed chars: Allowed chars:                ".to_string());
    }
}
