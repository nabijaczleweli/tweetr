tweetr(1) -- Self-hosted automatic tweet posting software
=========================================================

## SYNOPSIS

`tweetr` [OPTIONS] &lt;SUBMODULE&gt; [SUBMODULE_OPTIONS]

## DESCRIPTION

tweetr is a platform that allows you to create and queue tweets to be
shared when YOU want. You create content when you have time and then use FOSS
and NOT pay whatever-ridiculous amount of $$$ for posting them automatically.

In other words this is self-hosted automatic tweet posting software.

See the subcommands for more information:

  * tweetr-init(1) - authorising the application
  * tweetr-add-user(1) - adding and authorising users
  * tweetr-queue-tweet(1) - adding tweets to the queue
  * tweetr-start-daemon(1) - start the tweet-posting daemon

## OPTIONS

  -c --config-dir &lt;<algorithm>&gt;

    Directory with the configuration.

    The configuration directory contains all of tweetr's data.

    Default: $HOME/.tweetr

## EXAMPLES

  See the per-subcommand examples page

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/tweetr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/tweetr>&gt;
