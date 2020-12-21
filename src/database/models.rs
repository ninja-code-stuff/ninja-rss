use super::schema::feeds;

#[derive(Queryable, Debug)]
pub struct Feed {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "feeds"]
pub struct NewFeed<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}
