use hyper::Get;
use models::CursorIds;
use models::users::CursorUsers;

client!(FollowersClient, [
    (
        ids, Get,
        "https://api.twitter.com/1.1/followers/ids.json",
        [],
        [user_id: u64, screen_name: String, cursor: i64, count: i32],
        CursorIds
    ),
    (
        list, Get,
        "https://api.twitter.com/1.1/followers/list.json",
        [],
        [
            user_id: u64, screen_name: String, cursor: i64, count: i32,
            skip_status: bool, include_user_entities: bool
        ],
        CursorUsers
    )
]);