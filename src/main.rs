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
            ninja_rss::feeds::add(&ninja_rss::database::crud::get_conn()?, url)?;
        }
        ("list", Some(_)) => {
            println!(
                "{:#?}",
                ninja_rss::feeds::list(&ninja_rss::database::crud::get_conn()?)
            );
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

    #[test]
    fn test_add() {
        let matches = create_app().get_matches_from(vec!["ninja_rss", "add", "url"]);
        assert!(matches.subcommand_matches("add").is_some());
        let args = matches.subcommand_matches("add").unwrap();
        assert_eq!(args.value_of("url"), Some("url"));
    }

    #[test]
    fn test_list() {
        let matches = create_app().get_matches_from(vec!["ninja_rss", "list"]);
        assert!(matches.subcommand_matches("list").is_some());
    }
}
