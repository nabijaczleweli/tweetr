not-stakkr(1) -- Self-hosted automatic tweet posting software
=============================================================

## SYNOPSIS

`not-stakkr` [OPTIONS] &lt;SUBMODULE&gt; [SUBMODULE_OPTIONS]

## DESCRIPTION

not-stakkr is a platform that allows you to create and queue tweets to be
shared when YOU want. You create content when you have time and then use FOSS
and NOT pay whatever-ridiculous amount of $$$ for posting them automatically.

In other words this is self-hosted automatic tweet posting software.

See the subcommands for more information:

  * `not-stakkr-init(1)` - authorising the application
  * `not-stakkr-add-user(1)` - adding and authorising users
  * `not-stakkr-queue-tweet(1)` - adding tweets to the queue
  * `not-stakkr-start-daemon(1)` - start the tweet-posting daemon

## OPTIONS

  -c --config-dir &lt;<algorithm>&gt;

    Directory with the configuration.

    The configuration directory contains all of not-stakkr's data.

    Default: $HOME/.not-stakkr

## EXAMPLES

  See the per-subcommand examples page

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/not-stakkr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/not-stakkr>&gt;
