use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<SqliteConnectionManager>,
    pub cancel_flag: Arc<AtomicBool>,
    pub language: LanguageEnum,
}
#[derive(Clone)]
pub enum LanguageEnum {
    Chinese,
    English,
}
impl AppState {
    pub fn new() -> Result<AppState, anyhow::Error> {
        let home_dir = dirs::home_dir().ok_or(anyhow!("failed to get home directory"))?;
        let db_path = home_dir.join(".git_statistic.db");
        let manager = SqliteConnectionManager::file(db_path)
            .with_init(|c| c.execute_batch("PRAGMA journal_mode=wal;PRAGMA busy_timeout=60000;"));
        let pool = r2d2::Pool::new(manager)?;
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let language = LanguageEnum::English;
        // let connection = Connection::open(db_path)?;
        Ok(AppState {
            pool,
            cancel_flag,
            language,
        })
    }
}
