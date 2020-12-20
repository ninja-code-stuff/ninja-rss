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
