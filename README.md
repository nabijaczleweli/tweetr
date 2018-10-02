# tweetr [![TravisCI build status](https://travis-ci.org/nabijaczleweli/tweetr.svg?branch=master)](https://travis-ci.org/nabijaczleweli/tweetr) [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/kk34veg25wre0gqe/branch/master?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/tweetr/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://meritbadge.herokuapp.com/tweetr)](https://crates.io/crates/tweetr)
tweetr is a platform that allows you to create and queue tweets to be shared when YOU want. You create content when you have time and then use FOSS and NOT pay whatever-ridiculous amount of $$$ for posting them automatically.

## [Documentation](https://rawcdn.githack.com/nabijaczleweli/tweetr/doc/tweetr/index.html)
## [Manpages](https://rawcdn.githack.com/nabijaczleweli/tweetr/man/tweetr.1.html)

## Stock values
If you want to use an already set-up application decrypt (and [verify](https://heybase.io/nabijaczleweli)) `assets/stock-app.toml.asc` into `$CONFIG_DIR/app.toml`.

## Simple usage

First, enter the app data you got when making a Twitter app on https://dev.twitter.com (or use the [stock values](#stock-values) for a pre-set app):

```sh
tweetr init
App key: ...
App secret: ...
```

Then, authorise users by pasting the provided link into a logged-in browser then entering the resulting PIN:

```sh
tweetr add-user
Visit this URL: ...
Enter the PIN from that page: ...
```

Afterwards add tweets:

```sh
tweetr queue-tweets
Author (or empty to finish): ...
Tweet content: ...
Time to post the tweet (RFC2822 or RFC3339): ...

Author (or empty to finish):
```

At any point after `init` you can start the daemon, which will be posting tweets:

```sh
tweetr start-daemon
Posted tweet "..." scheduled for ... by ... at ... with ID ...
Posted tweet "..." scheduled for ... by ... at ... with ID ...
```

For more detailed information see the [manpages](#manpages).
