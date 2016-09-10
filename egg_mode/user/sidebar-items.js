initSidebarItems({"enum":[["Connection","Represents the ways a target account can be connected to another account."],["UserID","Convenience enum to generalize between referring to an account by numeric ID or by screen name."]],"fn":[["block","Block the given account with the authenticated user."],["blocks","Lookup the users that have been blocked by the authenticated user."],["blocks_ids","Lookup the users that have been blocked by the authenticated user, but only return their user IDs."],["follow","Follow the given account with the authenticated user, and set whether device notifications should be enabled."],["followers_ids","Lookup the users that follow a given account, but only return their user IDs."],["followers_of","Lookup the users that follow a given account."],["friends_ids","Lookup the users a given account follows, also called their \"friends\" within the API, but only return their user IDs."],["friends_no_retweets","Lookup the user IDs that the authenticating user has disabled retweets from."],["friends_of","Lookup the users a given account follows, also called their \"friends\" within the API."],["incoming_requests","Lookup the user IDs who have pending requests to follow the authenticated protected user."],["lookup","Lookup a set of Twitter users by either ID and screen name, as applicable."],["lookup_ids","Lookup a set of Twitter users by their numerical ID."],["lookup_names","Lookup a set of Twitter users by their screen name."],["mute","Mute the given user with the authenticated user."],["mutes","Lookup the users that have been muted by the authenticated user."],["mutes_ids","Lookup the users that have been muted by the authenticated user, but only return their user IDs."],["outgoing_requests","Lookup the user IDs with which the authenticating user has a pending follow request."],["relation","Lookup relationship settings between two arbitrary users."],["relation_lookup","Lookup the relations between the authenticated user and the given accounts."],["report_spam","Block the given account and report it for spam, with the authenticated user."],["search","Lookup users based on the given search term."],["show","Lookup user information for a single user."],["unblock","Unblock the given user with the authenticated user."],["unfollow","Unfollow the given account with the authenticated user."],["unmute","Unmute the given user with the authenticated user."],["update_follow","Update notification settings and reweet visibility for the given user."]],"struct":[["RelationLookup","Represents the relation the authenticated user has to a given account."],["RelationSource","Represents relationship settings between two Twitter accounts, from the perspective of the source user."],["RelationTarget","Represents relationship settings between two Twitter accounts, from the perspective of the target user."],["Relationship","Represents relationship settings between two Twitter accounts."],["TwitterUser","Represents a Twitter user."],["UserEntities","Container for URL entity information that may be paired with a user's profile."],["UserEntityDetail","Represents a collection of URL entity information paired with a specific user profile field."],["UserSearch","Represents an active user search."]]});