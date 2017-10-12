
use chrono::{DateTime,Utc};

pub struct Keyword {
    id: i64,
    uuid: String,
    date_created: DateTime<Utc>,
    name: String,
    parent: i64
}


impl Keyword {

}
