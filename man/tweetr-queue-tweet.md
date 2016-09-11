tweetr-queue-tweet(1) -- Self-hosted automatic tweet posting software - tweet queueing
==========================================================================================

## SYNOPSIS

`tweetr` [OPTIONS] `queue-tweet`

## DESCRIPTION

Add a tweet to the queue with the values provided via `stdin`.

Queued tweets can then be posted via `tweetr-start-daemon(1)`.

The required data (in order):

  * username - will need to be authorised by the time the tweet is posted,
  * tweet text content,
  * time to post in RFC2822, RFC3339 or a custom relative format.

Relative format:

  * `now` - current datetime
  * `in` *n* [`second`|`minute`|`hour`|`day`|`week`]{,`s`} (case-insensitive) -
      current datetime plus the specified amount of time

One can queue a tweet with multiple lines by suffixing a non-ending line
with a `\`, which can be escaped with a `\\`.

For description of `tweetr` itself see `tweetr(1)`.

## OPTIONS

  See `tweetr(1)`.

## EXAMPLES

  `tweetr queue-tweet`

  This will queue two tweets for the same time.

    Author (or empty to finish): nabijaczleweli
    Tweet content: Capitalism
    Time to post the tweet (RFC2822, RFC3339 or custom):
    2016-09-10T12:00:00+02:00

    Author (or empty to finish): nabijaczleweli
    Tweet content: Abolish the bourgeoisie!
    Time to post the tweet (RFC2822, RFC3339 or custom):
    Sat, 10 Sep 2016 12:00:00 +0200

    Author (or empty to finish):

  This will queue a multiline tweet with content *"Abolish\nthe\nburgeoisie!"*
  five minutes from now:

    Author (or empty to finish): nabijaczleweli
    Tweet content: Abolish\
                   the\
                   burgeoisie!
    Time to post the tweet (RFC2822, RFC3339 or custom): in 5 minutes

    Author (or empty to finish):

  This will queue a multiline tweet with content *"Escaped\"* an hour from now:

    Author (or empty to finish): nabijaczleweli
    Tweet content: Escaped\\
    Time to post the tweet (RFC2822, RFC3339 or custom): in 1 hour

    Author (or empty to finish):

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/tweetr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/tweetr>&gt;
