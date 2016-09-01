extern crate not_stakkr;

use self::not_stakkr::ops::User;
use std::env::temp_dir;
use std::fs;


#[test]
fn empty_eq() {
    trans_scaffold("empty_trans_eq", vec![]);
}

#[test]
fn single_eq() {
    trans_scaffold("single_trans_eq",
                   vec![User {
                            name: "nabijaczleweli".to_string(),
                            id: 481,
                            access_token_key: "481-FNlJkpZCE7a4Bbd7f1k65GtgaH7SmHlReWSESD4".to_string(),
                            access_token_secret: "GVQDq88qLtJ45KR6u44A6AljW31JSSippjdipQg6gPYE5".to_string(),
                        }]);
}

#[test]
fn multi_eq() {
    trans_scaffold("multi_trans_eq",
                   vec![User {
                            name: "nabijaczleweli".to_string(),
                            id: 481,
                            access_token_key: "481-FNlJkpZCE7a4Bbd7f1k65GtgaH7SmHlReWSESD4".to_string(),
                            access_token_secret: "GVQDq88qLtJ45KR6u44A6AljW31JSSippjdipQg6gPYE5".to_string(),
                        },
                        User {
                            name: "danerang".to_string(),
                            id: 334776,
                            access_token_key: "334776-WTRvsJI4DQgvzYwDYT8YYdEBxnpCQpQB2t4SrEK".to_string(),
                            access_token_secret: "qbOzYrP9bwcOEGAUnuiPVfA7JqAAsWhrd8DcbwDH5RwTA".to_string(),
                        },
                        User {
                            name: "LinesFromNLSS".to_string(),
                            id: 6695520,
                            access_token_key: "6695520-KhiuVzAS41GS0V3hCBA7VFnHHNdwQpUDCaNfiOn".to_string(),
                            access_token_secret: "3cx12ULmXYkhcnEiPXBbpoilLPdQOVd8KigUoPQmaw8f5".to_string(),
                        }]);
}


fn trans_scaffold(name: &str, users: Vec<User>) {
    let td = temp_dir().join("not-stakkr-test").join(format!("ops-user-{}", name));
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("users.toml");
    let _ = fs::remove_file(&tf);

    User::write(users.clone(), &tf);
    let read_users = User::read(&tf).unwrap();

    assert_eq!(users, read_users);
}
