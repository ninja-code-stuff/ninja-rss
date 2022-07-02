use std::error::Error;

use ninja_rss::rss_manager::RssManager;

pub fn init_rss_manager() -> Result<RssManager, Box<dyn Error>> {
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
