table! {
    books (book_id) {
        book_id -> Binary,
        title -> Varchar,
        author -> Binary,
        description -> Text,
        content_id -> Binary,
        tags -> Text,
        cover_url -> Varchar,
    }
}
