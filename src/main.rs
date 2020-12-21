use std::error::Error;

#[macro_use]
extern crate prettytable;

use ninja_rss::rss_manager::Feed;
use prettytable::Table;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(about, global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]))]
enum Opt {
    #[structopt(no_version, about = "Add rss feed by url")]
    Add { url: String },
    #[structopt(no_version, about = "Delete rss feed by id")]
    Del { id: i32 },
    #[structopt(no_version, about = "List rss feed")]
    List,
}

fn feed_to_table(feed: Feed) -> Table {
    let mut table = Table::new();
    table.add_row(row![bFg -> "Id", feed.id]);
    table.add_row(row![bFg -> "Title", b -> feed.title]);
    table.add_row(row![bFg -> "Description", feed.description]);
    table.add_row(row![bFg -> "Url", i -> feed.url]);
    table
}

fn feeds_to_table(feed_list: Vec<Feed>) -> Table {
    let mut table = Table::new();
    // table.add_row(row![bFg->"foobar", BriH2->"bar", "foo"]);
    table.set_titles(row![bFg -> "Id", bFg -> "Title", bFg -> "Description", bFg-> "Url"]);
    for feed in feed_list {
        table.add_row(row![
            feed.id,
            bc -> feed.title,
            feed.description,
            i -> feed.url,
        ]);
    }
    table
}

fn main() -> Result<(), Box<dyn Error>> {
    match Opt::from_args() {
        Opt::Add { url } => {
            feed_to_table(ninja_rss::rss_manager::get_rss_manager()?.add(&url)?).printstd();
        }
        Opt::Del { id } => {
            ninja_rss::rss_manager::get_rss_manager()?.delete(id)?;
        }
        Opt::List => {
            feeds_to_table(ninja_rss::rss_manager::get_rss_manager()?.list()?).printstd();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // Note: from_iter_safe provides Result
    //       unwrap() is used to fail the tests for err cases
    //       in prod helpful message is shown by not using safe

    #[test]
    fn test_add() {
        assert_eq!(
            Opt::from_iter_safe(vec!["ninja_rss", "add", "url"]).unwrap(),
            Opt::Add { url: "url".into() }
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Opt::from_iter_safe(vec!["ninja_rss", "list"]).unwrap(),
            Opt::List
        );
    }

    #[test]
    #[should_panic]
    fn test_list_with_args() {
        Opt::from_iter_safe(vec!["ninja_rss", "list", "args_added"]).unwrap();
    }

    #[test]
    fn test_delete() {
        assert_eq!(
            Opt::from_iter_safe(vec!["ninja_rss", "del", "3"]).unwrap(),
            Opt::Del { id: 3 }
        );
    }
}
