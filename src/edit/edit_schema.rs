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
