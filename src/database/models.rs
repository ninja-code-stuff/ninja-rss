use super::schema::feeds;

#[derive(Queryable, Debug)]
pub struct DbFeed {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "feeds"]
pub struct NewFeed<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
}
