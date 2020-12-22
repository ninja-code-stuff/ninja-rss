use super::models::{Feed, FeedItem, NewFeed, NewFeedItem};
use super::schema::{feed_items, feeds};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn update_schema(conn: &SqliteConnection) -> Result<(), Box<dyn Error>> {
    embed_migrations!();
    embedded_migrations::run(conn)?;
    Ok(())
}

pub fn get_conn() -> Result<SqliteConnection, Box<dyn Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL")?;
    let conn = SqliteConnection::establish(&url)?;
    Ok(conn)
}

pub fn get_all_feeds(conn: &SqliteConnection) -> Result<Vec<Feed>, Box<dyn Error>> {
    let res = feeds::table.get_results(conn)?;
    Ok(res)
}

pub fn create_feed(conn: &SqliteConnection, new_feed: &NewFeed) -> Result<Feed, Box<dyn Error>> {
    diesel::insert_into(feeds::table)
        .values(new_feed)
        .execute(conn)?;

    let post = feeds::table.order(feeds::id.desc()).first(conn)?;
    Ok(post)
}

pub fn delete_feed(conn: &SqliteConnection, id: i32) -> Result<(), Box<dyn Error>> {
    conn.execute("PRAGMA foreign_keys = ON")?;
    diesel::delete(feeds::table.find(id)).execute(conn)?;
    Ok(())
}

pub fn get_all_feed_items(
    conn: &SqliteConnection,
    feed_id: i32,
) -> Result<Vec<FeedItem>, Box<dyn Error>> {
    let res = feed_items::table
        .filter(feed_items::feed_id.eq(feed_id))
        .get_results(conn)?;
    Ok(res)
}

pub fn create_feed_items(
    conn: &SqliteConnection,
    items: &[NewFeedItem],
) -> Result<Vec<FeedItem>, Box<dyn Error>> {
    diesel::replace_into(feed_items::table)
        .values(items)
        .execute(conn)?;

    let items = feed_items::table
        .order(feed_items::id.desc())
        .limit(items.len() as i64)
        .get_results(conn)?;
    Ok(items)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_conn() -> SqliteConnection {
        let conn = SqliteConnection::establish(":memory:").unwrap();
        update_schema(&conn).unwrap();
        return conn;
    }

    #[test]
    fn test_add() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: "description",
        };
        let feed = create_feed(&conn, &new_feed).unwrap();
        assert_eq!(feed.id, 1);
    }

    #[test]
    fn test_list() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: "description",
        };
        create_feed(&conn, &new_feed).unwrap();
        create_feed(&conn, &new_feed).unwrap();

        let feeds = get_all_feeds(&conn).unwrap();
        assert_eq!(feeds.len(), 2);
    }

    #[test]
    fn test_delete() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: "description",
        };
        create_feed(&conn, &new_feed).unwrap();
        create_feed(&conn, &new_feed).unwrap();

        delete_feed(&conn, 2).unwrap();
        let feeds = get_all_feeds(&conn).unwrap();
        assert_eq!(feeds.len(), 1);

        delete_feed(&conn, 5).unwrap();
        assert_eq!(feeds.len(), 1);
    }

    #[test]
    fn test_feed_items_creation() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: "description",
        };
        let feed = create_feed(&conn, &new_feed).unwrap();
        let new_feed_item = NewFeedItem {
            feed_id: feed.id,
            guid: Some("hi"),
            title: "title",
            summary: "summary",
            link: "link",
        };
        create_feed_items(&conn, &[new_feed_item]).unwrap();
    }

    #[test]
    fn test_cascade_delete() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: "description",
        };
        let feed = create_feed(&conn, &new_feed).unwrap();
        let new_feed_item = NewFeedItem {
            feed_id: feed.id,
            guid: Some("hi"),
            title: "title",
            summary: "summary",
            link: "link",
        };
        create_feed_items(&conn, &[new_feed_item]).unwrap();

        let items = get_all_feed_items(&conn, feed.id).unwrap();
        assert_eq!(items.len(), 1);

        delete_feed(&conn, feed.id).unwrap();
        let items = get_all_feed_items(&conn, feed.id).unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_replace_into() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: "description",
        };
        let feed = create_feed(&conn, &new_feed).unwrap();
        let new_feed_item = NewFeedItem {
            feed_id: feed.id,
            guid: Some("hi"),
            title: "title",
            summary: "summary",
            link: "link",
        };
        create_feed_items(&conn, &[new_feed_item]).unwrap();

        let new_feed_item = NewFeedItem {
            feed_id: feed.id,
            guid: Some("hi"),
            title: "new title",
            summary: "summary",
            link: "link",
        };

        create_feed_items(&conn, &[new_feed_item]).unwrap();

        let items = get_all_feed_items(&conn, feed.id).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "new title");
    }
}
