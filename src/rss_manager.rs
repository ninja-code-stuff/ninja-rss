use super::database::crud;
pub use super::database::models::Feed;
use super::database::models::NewFeed;
use diesel::sqlite::SqliteConnection;
use rss::Channel;
use std::error::Error;

pub struct RssManager {
    conn: SqliteConnection,
}

pub fn get_rss_manager() -> Result<RssManager, Box<dyn Error>> {
    let conn = crud::get_conn()?;
    Ok(RssManager { conn })
}

impl RssManager {
    pub fn new(conn: SqliteConnection) -> Self {
        RssManager { conn }
    }

    #[cfg(not(test))]
    fn fetch(&self, uri: &str) -> Result<Channel, Box<dyn Error>> {
        let channel = Channel::from_url(uri)?;
        Ok(channel)
    }

    // Note: This swapping is done to avoid network call for testing
    #[cfg(test)]
    fn fetch(&self, uri: &str) -> Result<Channel, Box<dyn Error>> {
        let file = std::fs::File::open(uri)?;
        let reader = std::io::BufReader::new(file);
        let channel = Channel::read_from(reader)?;
        Ok(channel)
    }

    pub fn add(&self, uri: &str) -> Result<Feed, Box<dyn Error>> {
        let channel = self.fetch(uri)?;

        let new_feed = NewFeed {
            url: uri,
            title: channel.title(),
            description: channel.description(),
        };
        let db_feed = crud::create_feed(&self.conn, &new_feed)?;
        Ok(db_feed)
    }

    pub fn list(&self) -> Result<Vec<Feed>, Box<dyn Error>> {
        let db_feeds = crud::get_all_feeds(&self.conn)?;
        Ok(db_feeds.into_iter().collect())
    }

    pub fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        crud::delete_feed(&self.conn, id)
    }
}
