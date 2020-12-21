use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    match Opt::from_args() {
        Opt::Add { url } => {
            ninja_rss::rss_manager::get_rss_manager()?.add(&url)?;
        }
        Opt::Del { id } => {
            ninja_rss::rss_manager::get_rss_manager()?.delete(id)?;
        }
        Opt::List => {
            println!("{:#?}", ninja_rss::rss_manager::get_rss_manager()?.list()?);
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
