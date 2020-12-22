use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};
use ninja_rss::{rss_manager::Feed, rss_manager::FeedItem};

pub fn feed_to_table(feed: Feed) -> Table {
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

pub fn feeds_to_table(feed_list: Vec<Feed>) -> Table {
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

pub fn feed_items_to_table(feed_list: Vec<FeedItem>) -> Table {
    let set_header_style = |header: &str| {
        Cell::new(header)
            .add_attribute(Attribute::Bold)
            .fg(Color::Green)
    };
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(
        vec!["Id", "Title", "Summary", "Link"]
            .into_iter()
            .map(set_header_style),
    );
    for feed in feed_list {
        table.add_row(vec![
            feed.id.to_string(),
            feed.title,
            feed.summary,
            feed.link,
        ]);
    }
    table
}
