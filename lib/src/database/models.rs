use super::schema::{feed_items, feeds};

#[derive(Queryable, Debug)]
pub struct Feed {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "feeds"]
pub struct NewFeed<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Debug)]
pub struct FeedItem {
    pub id: i32,
    pub guid: Option<String>,
    pub title: String,
    pub summary: String,
    pub link: String,
    pub feed_id: i32,
}

#[derive(Insertable)]
#[table_name = "feed_items"]
pub struct NewFeedItem<'a> {
    pub guid: Option<&'a str>,
    pub title: &'a str,
    pub summary: &'a str,
    pub link: &'a str,
    pub feed_id: i32,
}
