tweetr-add-user(1) -- Self-hosted automatic tweet posting software - user authorisation
=======================================================================================

## SYNOPSIS

`tweetr` [OPTIONS] `add-user` [ADD_USER_OPTIONS]

## DESCRIPTION

Add and authorise a user via Twitter's PIN authorisation pipeline.

Authorising a new user with the same name doesn't require forcing as the new
tokens are the only valid ones at that point.

For description of `tweetr` itself see `tweetr(1)`.

## OPTIONS

  See `tweetr(1)`.

## ADD_USER_OPTIONS

  -v --verbose

    Log network events and user tokens.

    By default the add-user subsystem will print enough of information for
    normal usage, but usign this can help one troubleshoot network problems.

## EXAMPLES

  `tweetr add-user`

  Normal:

    Visit this URL: https://api.twitter.com/oauth/authorize?oauth_token=3JBg-BAV
    Enter the PIN from that page: 9530177

    Successfully authenticated user nabijaczleweli#1246428073

  Entering the wrong PIN:

    Visit this URL: https://api.twitter.com/oauth/authorize?oauth_token=3JBg-BAV
    Enter the PIN from that page: 9530178
    Twitter API error: Error status received: 401 Unauthorized

  `tweetr add-user -v`

  This will also print network data and user access tokens:

    Getting request token... DONE
    Getting authorisation URL... DONE

    Visit this URL: https://api.twitter.com/oauth/authorize?oauth_token=3JBg-BAV
    Enter the PIN from that page: 9530177

    Getting access token... DONE

    Successfully authenticated user nabijaczleweli#1246428073
    Access tokens:
      Key   : 1246428073-KhiuVzAS41GS0V3hCBA7VFnHHNdwQpUDCaNfiOn
      Secret: 3cx12ULmXYkhcnEiPXBbpoilLPdQOVd8KigUoPQmaw8f5

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/tweetr/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/tweetr>&gt;
