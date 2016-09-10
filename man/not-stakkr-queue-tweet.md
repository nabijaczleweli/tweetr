not-stakkr-queue-tweet(1) -- Self-hosted automatic tweet posting software - tweet queueing
==========================================================================================

## SYNOPSIS

`not-stakkr` [OPTIONS] `queue-tweet`

## DESCRIPTION

Add a tweet to the queue with the values provided via `stdin`.

Queued tweets can then be posted via `not-stakkr-start-daemon(1)`.

The required data (in order):

  * username - will need to be authorised by the time the tweet is posted,
  * tweet text content,
  * time to post in RFC2822 or RFC3339 format.

For description of `not-stakkr` itself see `not-stakkr(1)`.

## OPTIONS

  See `not-stakkr(1)`.

## EXAMPLES

  `not-stakkr queue-tweet`

  This will queue two tweets for the same time.

    Author (or empty to finish): nabijaczleweli
    Tweet content: Capitalism
    Time to post the tweet (RFC2822 or RFC3339): 2016-09-10T12:00:00+02:00

    Author (or empty to finish): nabijaczleweli
    Tweet content: Abolish the bourgeoisie!
    Time to post the tweet (RFC2822 or RFC3339): Sat, 10 Sep 2016 12:00:00 +0200

    Author (or empty to finish):

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/not-stakkr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/not-stakkr>&gt;
