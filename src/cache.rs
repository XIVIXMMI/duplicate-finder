use rusqlite::{Connection, params};

pub fn init_db() -> Connection {
    let conn = Connection::open("cachedb").expect("Cannot open DB");
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS file_cache (
            path        TEXT PRIMARY KEY,
            modified_at  INTEGER NOT NULL,
            hash       TEXT NOT NULL 
        );
        ",
    )
    .expect("Cannot create table");
    conn
}

pub fn get_cached_hash(conn: &Connection, path: &str, modified_at: u64) -> Option<String> {
    conn.query_row(
        "SELECT hash FROM file_cache WHERE path = ?1 AND modified_at = ?2",
        params![path, modified_at],
        |row| row.get(0),
    )
    .ok()
}

pub fn save_hash(conn: &Connection, path: &str, modified_at: u64, hash: &str) {
    conn.execute(
        "INSERT OR REPLACE INTO file_cache (path, modified_at, hash) VALUES (?1,?2,?3)",
        params![path, modified_at, hash],
    )
    .expect("Cannot save hash");
}