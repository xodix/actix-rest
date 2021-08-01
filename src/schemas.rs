table! {
    users (user_id) {
        user_id -> Binary,
        name -> Varchar,
        email -> Varchar,
        phone_num -> Integer,
        img_url -> Varchar,
        password -> Varchar,
        favourites -> Text,
    }
}

table! {
    bookmarks (bookmark_id) {
        bookmark_id -> Binary,
        book_id -> Binary,
        user_id -> Binary,
        ended_on -> Integer,
    }
}

table! {
    edits (edit_id) {
        edit_id -> Binary,
        start -> Integer,
        end -> Integer,
        last_edit_id -> Integer,
        md_content -> Text,
        book_id -> Binary,
    }
}
