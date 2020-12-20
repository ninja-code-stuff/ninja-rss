use super::database::crud;
use super::database::models::{DbFeed, NewFeed};
use diesel::sqlite::SqliteConnection;
use rss::Channel;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
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

pub trait FeedFetcher {
    fn fetch(&self, uri: &str) -> Result<Channel, Box<dyn Error>>;
}

pub struct UrlFeedFetcher;

impl FeedFetcher for UrlFeedFetcher {
    fn fetch(&self, uri: &str) -> Result<Channel, Box<dyn Error>> {
        let channel = Channel::from_url(uri)?;
        Ok(channel)
    }
}

pub struct FileFeedFetcher;

impl FeedFetcher for FileFeedFetcher {
    fn fetch(&self, uri: &str) -> Result<Channel, Box<dyn Error>> {
        let file = File::open(uri)?;
        let reader = BufReader::new(file);
        let channel = Channel::read_from(reader)?;
        Ok(channel)
    }
}

pub struct RssManager<T: FeedFetcher> {
    conn: SqliteConnection,
    fetcher: T,
}

pub fn get_rss_manager() -> Result<RssManager<UrlFeedFetcher>, Box<dyn Error>> {
    let conn = crud::get_conn()?;
    let fetcher = UrlFeedFetcher;
    Ok(RssManager { conn, fetcher })
}

impl<T: FeedFetcher> RssManager<T> {
    pub fn new(conn: SqliteConnection, fetcher: T) -> Self {
        RssManager { conn, fetcher }
    }

    pub fn add(&self, uri: &str) -> Result<Feed, Box<dyn Error>> {
        let channel = self.fetcher.fetch(uri)?;

        let new_feed = NewFeed {
            url: uri,
            title: channel.title(),
            description: Some(channel.description()),
        };
        let db_feed = crud::create_feed(&self.conn, &new_feed)?;
        Ok(convert(db_feed))
    }

    pub fn list(&self) -> Result<Vec<Feed>, Box<dyn Error>> {
        let db_feeds = crud::get_all_feeds(&self.conn)?;
        Ok(db_feeds.into_iter().map(convert).collect())
    }
}
