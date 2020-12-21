use std::error::Error;

use clap::App;

#[macro_use]
extern crate clap;

fn create_app() -> App<'static, 'static> {
    clap_app!(ninja_rss =>
        (version: crate_version!())
        (about: crate_description!())
        (@setting VersionlessSubcommands)
        (@subcommand add =>
         (about: "add rss url")
         (@arg url: +required "url to be added")
        )
        (@subcommand del =>
         (about: "delete rss by id")
         (@arg id: +required "id to be deleted")
        )
        (@subcommand list =>
         (about: "list rss url")
        )
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = create_app().get_matches();
    match matches.subcommand() {
        ("add", Some(arg)) => {
            let url = arg
                .value_of("url")
                .ok_or("could not get url argument for add")?;
            ninja_rss::rss_manager::get_rss_manager()?.add(url)?;
        }
        ("del", Some(arg)) => {
            let id = arg
                .value_of("id")
                .ok_or("could not get url argument for add")?;
            ninja_rss::rss_manager::get_rss_manager()?.delete(id.parse().unwrap())?;
        }
        ("list", Some(_)) => {
            println!("{:#?}", ninja_rss::rss_manager::get_rss_manager()?.list()?);
        }
        _ => {
            eprintln!("{}", matches.usage());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // Note: get_matches_from_safe provides and Result
    //       unwrap() is used to fail the tests for err cases
    //       in prod helpful message is shown by not using safe

    #[test]
    fn test_add() {
        let matches = create_app()
            .get_matches_from_safe(vec!["ninja_rss", "add", "url"])
            .unwrap();
        assert!(matches.subcommand_matches("add").is_some());
        let args = matches.subcommand_matches("add").unwrap();
        assert_eq!(args.value_of("url"), Some("url"));
    }

    #[test]
    fn test_list() {
        let matches = create_app()
            .get_matches_from_safe(vec!["ninja_rss", "list"])
            .unwrap();
        assert!(matches.subcommand_matches("list").is_some());
    }

    #[test]
    fn test_delete() {
        let matches = create_app()
            .get_matches_from_safe(vec!["ninja_rss", "del", "3"])
            .unwrap();
        assert!(matches.subcommand_matches("del").is_some());
        let args = matches.subcommand_matches("del").unwrap();
        assert_eq!(args.value_of("id"), Some("3"));
    }

    #[test]
    #[should_panic]
    fn test_wrong_case() {
        create_app()
            .get_matches_from_safe(vec!["ninja_rss", "wrong_case"])
            .unwrap();
    }
}
