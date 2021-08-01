table! {
    bookmarks (bookmark_id) {
        bookmark_id -> Binary,
        book_id -> Binary,
        user_id -> Binary,
        ended_on -> Integer,
    }
}
