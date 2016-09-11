tweetr-init(1) -- Self-hosted automatic tweet posting software - setting application
====================================================================================

## SYNOPSIS

`tweetr` [OPTIONS] `init` [INIT_OPTIONS]

## DESCRIPTION

Initialise the global application data with the values provided via `stdin`.

For description of `tweetr` itself see `tweetr(1)`.

## OPTIONS

  See `tweetr(1)`.

## INIT_OPTIONS

  -f --force

    Override current app configuration.

    By default the init subsystem will fail if app data already exists
    to prevent data loss, use this option to override that.

## EXAMPLES

  `tweetr init` [`-f`]

  The basic form of initialisation, will fail if an application is already
  initialised and `-f` was not provided.

  Full in/output pass:

    App key: qdPD7N8CcPYDKiNv81QWNWaHK
    App secret: U9A5CM1LzwNliBiHGPIJyx6tFYAGVr3bCMbVkWKu8Zb13kHD4p

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/tweetr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/tweetr>&gt;
