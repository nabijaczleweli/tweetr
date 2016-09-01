extern crate not_stakkr;

use self::not_stakkr::ops::AppTokens;
use std::env::temp_dir;
use std::fs;


#[test]
fn trans_eq() {
    let td = temp_dir().join("not-stakkr-test").join("ops-token-trans_eq");
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("app.toml");
    let _ = fs::remove_file(&tf);

    let tokens = AppTokens {
        key: "qzuqpwr101q4RtK9mDorI9ndm".to_string(),
        secret: "HW4YG3Kdcap5ovcZ5fZfBJFedKR6GQe9MtZDS9Gm34hXiirkU5".to_string(),
    };
    tokens.write(&tf);
    let read_tokens = AppTokens::read(&tf).unwrap();

    assert_eq!(tokens, read_tokens);
}
