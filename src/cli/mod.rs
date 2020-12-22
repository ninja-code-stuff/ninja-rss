use std::error::Error;
mod display;
mod opt;
mod setup;

use opt::Opt;
use structopt::StructOpt;

extern crate comfy_table;
extern crate structopt;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let rss_manger = crate::setup::init_rss_manager()?;
    match opt {
        Opt::Add { url } => {
            println!("{}", display::feed_to_table(rss_manger.add(&url)?));
        }
        Opt::Del { id } => {
            rss_manger.delete(id)?;
        }
        Opt::List => {
            println!("{}", display::feeds_to_table(rss_manger.list()?));
        }
    }
    Ok(())
}
