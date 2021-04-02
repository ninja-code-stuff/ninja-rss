use structopt::clap::AppSettings;

use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum FeedOpt {
    #[structopt(about = "List rss feed")]
    List,
    #[structopt(about = "Refresh rss feed")]
    Refresh,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(about, global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]))]
pub enum Opt {
    #[structopt(about = "Add rss feed by url")]
    Add { url: String },
    #[structopt(about = "Delete rss feed by id")]
    Del { id: i32 },
    #[structopt(about = "List rss feed")]
    List,
    #[structopt(about = "Feed Operations")]
    Feed {
        #[structopt(short, long, about = "Feed Id")]
        feed_id: i32,
        #[structopt(subcommand)]
        feed_opt: FeedOpt,
    },
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
