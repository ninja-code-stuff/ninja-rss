use super::database::crud;
pub use super::database::models::{Feed, FeedItem};
use super::database::models::{NewFeed, NewFeedItem};
use diesel::sqlite::SqliteConnection;
use rss::{Channel, Item};
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

    pub fn update_schema(&self) -> Result<(), Box<dyn Error>> {
        crud::update_schema(&self.conn)
    }

    #[cfg(not(test))]
    fn fetch(&self, uri: &str) -> Result<Channel, Box<dyn Error>> {
        // https://users.rust-lang.org/t/lightweight-alternative-for-reqwest/33601/21
        let mut content = Vec::new();
        ::http_req::request::get(uri, &mut content)?;
        Ok(Channel::read_from(&content[..])?)
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
        self.add_items(db_feed.id, channel.items())?;
        Ok(db_feed)
    }

    fn add_items(&self, feed_id: i32, items: &[Item]) -> Result<(), Box<dyn Error>> {
        let items: Vec<NewFeedItem> = items
            .iter()
            .map(|it| NewFeedItem {
                feed_id,
                guid: it.guid().map(|x| x.value()),
                title: it.title().unwrap_or(""),
                summary: it.description().unwrap_or(""),
                link: it.link().unwrap_or(""),
            })
            .collect();
        crud::create_feed_items(&self.conn, &items)?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Feed>, Box<dyn Error>> {
        let db_feeds = crud::get_all_feeds(&self.conn)?;
        Ok(db_feeds.into_iter().collect())
    }

    pub fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        crud::delete_feed(&self.conn, id)
    }

    pub fn get_items(&self, id: i32) -> Result<Vec<FeedItem>, Box<dyn Error>> {
        Ok(crud::get_all_feed_items(&self.conn, id)?)
    }

    pub fn refresh(&self, id: i32) -> Result<Feed, Box<dyn Error>> {
        let feed = crud::get_feed(&self.conn, id)?;
        let ch = self.fetch(&feed.url)?;
        let new_feed = Feed {
            title: ch.title().to_string(),
            description: ch.description().to_string(),
            ..feed
        };
        crud::update_feed(&self.conn, &new_feed)?;
        self.add_items(id, ch.items())?;
        Ok(new_feed)
    }
}
