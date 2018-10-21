table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        hash -> Text,
        salt -> Varchar,
        phone -> Varchar,
    }
}
