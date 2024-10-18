// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod common_tools;
mod sql_lite;
use log::LevelFilter;
use sql_lite::connection::AppTrayMenu;
mod service;
mod vojo;
use crate::service::cmd::*;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use crate::sql_lite::connection::AppState;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::MouseButton;
use tauri::tray::MouseButtonState;
use tauri::tray::TrayIconBuilder;
use tauri::tray::TrayIconEvent;

use tauri::Manager;
fn main() -> Result<(), anyhow::Error> {
    let app_state = AppState::new()?;
    tauri::Builder::default()
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
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit".to_string(), "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show".to_string(), "Show", true, None::<&str>)?;

            let app_statex = app.state::<AppState>();

            {
                let mut tray_menu = app_statex.app_tray_menu.lock().unwrap();
                *tray_menu = Some(AppTrayMenu {
                    quit_menu: quit.clone(),
                    show_menu: show.clone(),
                });
            }

            let menu = Menu::with_items(app, &[&show, &quit])?;
            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        info!("quit menu item was clicked");
                        app.exit(0);
                    }
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {
                        info!("menu item {:?} not handled", event);
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        info!("left click pressed and released");
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        // info!("unhandled event {event:?}");
                    }
                })
                .build(app)?;

            Ok(())
        })
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
            set_language,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
