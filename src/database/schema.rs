table! {
    feed_items (id) {
        id -> Integer,
        guid -> Nullable<Text>,
        title -> Text,
        summary -> Text,
        link -> Text,
        feed_id -> Integer,
    }
}

table! {
    feeds (id) {
        id -> Integer,
        url -> Text,
        title -> Text,
        description -> Text,
    }
}

joinable!(feed_items -> feeds (feed_id));

allow_tables_to_appear_in_same_query!(
    feed_items,
    feeds,
);
