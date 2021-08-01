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
