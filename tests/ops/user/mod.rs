extern crate egg_mode;
extern crate tweetr;

use self::tweetr::ops::User;
use self::egg_mode::Token;

mod trans;


#[test]
fn convert_from_raw_token() {
    let user = User {
        name: "nabijaczleweli".to_string(),
        id: 481,
        access_token_key: "481-FNlJkpZCE7a4Bbd7f1k65GtgaH7SmHlReWSESD4".to_string(),
        access_token_secret: "GVQDq88qLtJ45KR6u44A6AljW31JSSippjdipQg6gPYE5".to_string(),
    };
    assert_eq!(User::from_raw_access_token((Token::new(&user.access_token_key[..], &user.access_token_secret[..]), user.id, user.name.clone())),
               user);
}
