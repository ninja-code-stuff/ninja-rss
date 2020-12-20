use super::models::{DbFeed, NewFeed};
use super::schema::feeds;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn get_conn() -> Result<SqliteConnection, Box<dyn Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL")?;
    let conn = SqliteConnection::establish(&url)?;
    Ok(conn)
}

pub fn get_all_feeds(conn: &SqliteConnection) -> Result<Vec<DbFeed>, Box<dyn Error>> {
    let res = feeds::table.get_results(conn)?;
    Ok(res)
}

pub fn create_feed(conn: &SqliteConnection, new_feed: &NewFeed) -> Result<DbFeed, Box<dyn Error>> {
    diesel::insert_into(feeds::table)
        .values(new_feed)
        .execute(conn)?;

    let post = feeds::table.order(feeds::id.desc()).first(conn)?;
    Ok(post)
}

pub fn delete_feed(conn: &SqliteConnection, id: i32) -> Result<(), Box<dyn Error>> {
    diesel::delete(feeds::table.find(id)).execute(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_conn() -> SqliteConnection {
        let conn = SqliteConnection::establish(":memory:").unwrap();
        embed_migrations!();
        embedded_migrations::run(&conn).unwrap();
        return conn;
    }

    #[test]
    fn test_add() {
        let conn = get_conn();
        let new_feed = NewFeed {
            url: "url",
            title: "title",
            description: Some("description"),
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
            description: Some("description"),
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
            description: Some("description"),
        };
        create_feed(&conn, &new_feed).unwrap();
        create_feed(&conn, &new_feed).unwrap();

        delete_feed(&conn, 2).unwrap();
        let feeds = get_all_feeds(&conn).unwrap();
        assert_eq!(feeds.len(), 1);

        delete_feed(&conn, 5).unwrap();
        assert_eq!(feeds.len(), 1);
    }
}
