CREATE TABLE feed_items(
  id     INTEGER PRIMARY KEY NOT NULL,
  guid TEXT,
  title TEXT NOT NULL,
  summary   TEXT NOT NULL,
  link TEXT NOT NULL,
  feed_id INTEGER REFERENCES feeds(id) ON DELETE CASCADE NOT NULL,
  UNIQUE(feed_id, guid)
)
