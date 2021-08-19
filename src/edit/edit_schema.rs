table! {
    edits (edit_id) {
        edit_id -> Binary,
        edit_date-> Date,
        start -> Integer,
        end -> Integer,
        md_content -> Text,
        book_id -> Binary,
    }
}
