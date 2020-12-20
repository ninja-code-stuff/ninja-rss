use super::database::crud;
use super::database::models::{DbFeed, NewFeed};
use diesel::sqlite::SqliteConnection;
use rss::Channel;
use std::error::Error;

pub struct Feed {
    pub id: u64,
    pub url: String,
    pub title: String,
    pub description: String,
}

fn convert(db_feed: DbFeed) -> Feed {
    Feed {
        id: db_feed.id as u64,
        url: db_feed.url,
        title: db_feed.title,
        description: db_feed.description.unwrap(),
    }
}

pub async fn add(conn: &SqliteConnection, url: &str) -> Result<Feed, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    let new_feed = NewFeed {
        url: url,
        title: channel.title(),
        description: Some(channel.description()),
    };
    let db_feed = crud::create_feed(conn, &new_feed)?;
    Ok(convert(db_feed))
}

pub fn list(conn: &SqliteConnection) -> Result<Vec<Feed>, Box<dyn Error>> {
    let db_feeds = crud::get_all_feeds(conn)?;
    Ok(db_feeds.into_iter().map(convert).collect())
}
