#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(ninja_rss =>
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
    .get_matches();

    match matches.subcommand() {
        ("add", Some(arg)) => { println!("add reached with url {}", arg.value_of("url").unwrap())}
        ("list", Some(_)) => {println!("list reached") }
        _ => {
            eprintln!("{}", matches.usage());
        }
    }
}


