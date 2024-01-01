use rusqlite::{params, Connection, Result};
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
pub struct SqlLite {
    pub connection: Connection,
}

pub struct SqlLiteState(pub Mutex<SqlLite>);

impl SqlLite {
    pub fn new() -> Result<SqlLite, anyhow::Error> {
        let home_dir = dirs::home_dir().ok_or(anyhow!("failed to get home directory"))?;
        let db_path = home_dir.join(".git_statistic.db");
        let connection = Connection::open(db_path)?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS git_base_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            age    INTEGER NOT NULL, 
            active_days  INTEGER NOT NULL,
            total_files_count INTEGER NOT NULL,
            total_lines_count INTEGER NOT NULL,
            total_added_count INTEGER NOT NULL,
            total_deleted_count INTEGER NOT NULL,
            total_commits_count INTEGER NOT NULL,
            authors_count INTEGER NOT NULL
            )",
            params![],
        )?;

        Ok(SqlLite { connection })
    }
}
