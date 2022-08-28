table! {
    post (id) {
        author_id -> Uuid,
        id -> Uuid,
        title -> Varchar,
        text -> Text,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Varchar,
        hash -> Varchar,
    }
}

joinable!(post -> user (author_id));

allow_tables_to_appear_in_same_query!(
    post,
    user,
);
