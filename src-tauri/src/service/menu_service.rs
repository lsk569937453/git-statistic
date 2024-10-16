use tauri::State;

use crate::sql_lite::connection::AppState;

pub fn set_language_with_error(
    state: State<AppState>,
    language: String,
) -> Result<(), anyhow::Error> {
    let mut lock = state
        .app_tray_menu
        .lock()
        .map_err(|e| anyhow!("failed to lock mutex: {}", e))?;
    info!("set language {}", language);
    if let Some(tray_menu) = &mut *lock {
        let (quit_text, show_text) = if language == "zh" {
            ("退出", "显示")
        } else {
            ("Quit", "Show")
        };

        tray_menu.quit_menu.set_text(quit_text)?;
        tray_menu.show_menu.set_text(show_text)?;
    }

    Ok(())
}
