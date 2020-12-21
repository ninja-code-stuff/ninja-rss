#[macro_use]
extern crate diesel;

#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;

pub(self) mod database;
pub mod rss_manager;
