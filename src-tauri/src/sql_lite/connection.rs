use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tauri::menu::MenuItem;
use tauri::Wry;
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<SqliteConnectionManager>,
    pub cancel_flag: Arc<AtomicBool>,
    pub app_tray_menu: Arc<Mutex<Option<AppTrayMenu>>>,
}
#[derive(Clone)]
pub struct AppTrayMenu {
    pub quit_menu: MenuItem<Wry>,
    pub show_menu: MenuItem<Wry>,
}
impl AppState {
    pub fn new() -> Result<AppState, anyhow::Error> {
        let home_dir = dirs::home_dir().ok_or(anyhow!("failed to get home directory"))?;
        let db_path = home_dir.join(".git_statistic.db");
        let manager = SqliteConnectionManager::file(db_path)
            .with_init(|c| c.execute_batch("PRAGMA journal_mode=wal;PRAGMA busy_timeout=60000;"));
        let pool = r2d2::Pool::new(manager)?;
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let app_tray_menu = Arc::new(Mutex::new(None));
        // let connection = Connection::open(db_path)?;
        Ok(AppState {
            pool,
            cancel_flag,
            app_tray_menu,
        })
    }
}
