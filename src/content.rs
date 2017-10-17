
use rusqlite::Connection;

#[derive(Debug,Default)]
pub struct Content {
    filter: Option<String>,
    sort_type: Option<String>,
    sort_direction: Option<String>,
    smart_collection: Option<String>,
}

impl Content {
    pub fn from_db(conn: &Connection, table: &str, container_col: &str,
                   container_id: i64) -> Content {
        let mut content = Content::default();

        let query = format!("SELECT content,owningModule from {} where {}=?1",
                            table, container_col);
        if let Ok(mut stmt) = conn.prepare(&query) {
            let mut rows = stmt.query(&[&container_id]).unwrap();
            while let Some(Ok(row)) = rows.next() {
                let value = row.get_checked(0);
                let owning_module: String = row.get(1);
                match owning_module.as_str() {
                    "com.adobe.ag.library.filter" =>
                        content.filter = value.ok(),
                    "com.adobe.ag.library.sortType" =>
                        content.sort_type = value.ok(),
                    "com.adobe.ag.library.sortDirection" =>
                        content.sort_direction = value.ok(),
                    "ag.library.smart_collection" =>
                        content.smart_collection = value.ok(),
                    _ => (),
                };
            }
        }
        content
    }
}
