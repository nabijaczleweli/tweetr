not-stakkr-start-daemon(1) -- Self-hosted automatic tweet posting software - tweet posting
==========================================================================================

## SYNOPSIS

`not-stakkr` [OPTIONS] `start-daemon` [START_DAEMON_OPTIONS]

## DESCRIPTION

Start the daemon that will post queued tweets to Twitter,

The user needs to be authorised for the application set via
`not-stakkr-init(1)` before posting a tweet from its account, use
`not-stakkr-add-user(1)` to do that.
Queue tweets using `not-stakkr-queue-tweet(1)`.

For description of `not-stakkr` itself see `not-stakkr(1)`.

## OPTIONS

  See `not-stakkr(1)`.

## START_DAEMON_OPTIONS

  -v --verbose

    Log network accesses, useful if your internet connection is failing.

  --delay &lt;<sleep_time>&gt;

    Time to wait between checking for and posting tweets.

    Unit: milliseconds.
    Default: 60000.

## EXAMPLES

  `not-stakkr start-daemon`

    Posted tweet "Capitalism" scheduled for
    2016-09-09T00:33:30+02:00 by not_stakkr_test at 2016-09-10T10:49:38+00:00
    with ID 774560457755590656
    Posted tweet "Abolish the burgeoisie!" scheduled for
    2016-09-10T00:33:30+02:00 by not_stakkr_test at 2016-09-10T10:49:39+00:00
    with ID 774560460511248384

  `not-stakkr start-daemon -v`

    Posting tweet scheduled for 2016-09-09T00:33:30+02:00... SUCCESS
    Posted tweet "Capitalism" scheduled for
    2016-09-09T00:33:30+02:00 by not_stakkr_test at 2016-09-10T10:53:11+00:00
    with ID 774561353273147392
    Posting tweet scheduled for 2016-09-10T00:33:30+02:00... SUCCESS
    Posted tweet "Abolish the burgeoisie!" scheduled for
    2016-09-10T00:33:30+02:00 by not_stakkr_test at 2016-09-10T10:53:12+00:00
    with ID 774561355886108674

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/not-stakkr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/not-stakkr>&gt;
