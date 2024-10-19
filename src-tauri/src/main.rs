// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod common_tools;
mod sql_lite;
use log::LevelFilter;
mod service;
mod vojo;
use crate::service::cmd::*;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use crate::sql_lite::connection::AppState;

use tauri::Manager;
fn main() -> Result<(), anyhow::Error> {
    let app_state = AppState::new()?;
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(app_state)
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.clone() {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .plugin(
            //C:\Users\56993\AppData\Local\com.lsk.gitstatistic\logs`
            tauri_plugin_log::Builder::default()
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_about_version,
            get_base_info,
            init_git_async,
            get_init_status,
            get_commit_info,
            get_authors_info,
            get_files_info,
            get_line_info,
            get_tag_info,
            cancel_init_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
