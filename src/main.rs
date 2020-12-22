use std::error::Error;

extern crate comfy_table;

use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};
use ninja_rss::rss_manager::{RssManager, Feed};
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(about, global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]))]
enum Opt {
    #[structopt(about = "Add rss feed by url")]
    Add { url: String },
    #[structopt(about = "Delete rss feed by id")]
    Del { id: i32 },
    #[structopt(about = "List rss feed")]
    List,
}

fn feed_to_table(feed: Feed) -> Table {
    let create_row = |header: &str, value: &str| {
        vec![
            Cell::new(header)
                .add_attribute(Attribute::Bold)
                .fg(Color::Green),
            Cell::new(value),
        ]
    };
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.add_row(create_row("Id", &feed.id.to_string()));
    table.add_row(create_row("Title", &feed.title));
    table.add_row(create_row("Description", &feed.description));
    table.add_row(create_row("Url", &feed.url));
    table
}

fn feeds_to_table(feed_list: Vec<Feed>) -> Table {
    let set_header_style = |header: &str| {
        Cell::new(header)
            .add_attribute(Attribute::Bold)
            .fg(Color::Green)
    };
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(
        vec!["Id", "Title", "Description", "Url"]
            .into_iter()
            .map(set_header_style),
    );
    for feed in feed_list {
        table.add_row(vec![
            feed.id.to_string(),
            feed.title,
            feed.description,
            feed.url,
        ]);
    }
    table
}

fn init_rss_manager() -> Result<RssManager, Box<dyn Error>>{
    // TODO: pass this as parameter
    let mut local_path = dirs::data_local_dir().unwrap();
    local_path.push("ninja_rss");
    std::fs::create_dir_all(&local_path)?;
    local_path.push("rss.db");
    std::env::set_var("DATABASE_URL", local_path.into_os_string());

    // TODO: try to migrate only on update
    let rss_manger = ninja_rss::rss_manager::get_rss_manager()?;
    rss_manger.update_schema()?;
    Ok(rss_manger)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let rss_manger = init_rss_manager()?;
    match opt {
        Opt::Add { url } => {
            println!("{}", feed_to_table(rss_manger.add(&url)?));
        }
        Opt::Del { id } => {
            rss_manger.delete(id)?;
        }
        Opt::List => {
            println!("{}", feeds_to_table(rss_manger.list()?));
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
