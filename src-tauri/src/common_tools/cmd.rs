use std::thread;

use crate::common_tools::about::get_about_version_with_error;
use crate::common_tools::base_response::BaseResponse;

use crate::common_tools::git::get_base_info_with_error;
use crate::common_tools::git::get_commit_info_with_error;
use crate::common_tools::git::init_git_with_error;
use crate::service::menu_service::set_language_with_error;
use crate::sql_lite::connection::AppState;
use git2::Repository;
use tauri::State;

use super::git::get_authors_info_with_error;
use super::git::get_files_info_with_error;
use super::git::get_init_status_with_error;
use super::git::get_tags_info_with_error;
#[tauri::command]
pub fn get_base_info(state: State<AppState>) -> String {
    match get_base_info_with_error(state) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn get_commit_info(state: State<AppState>) -> String {
    match get_commit_info_with_error(state) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn get_authors_info(state: State<AppState>) -> String {
    match get_authors_info_with_error(state) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn get_files_info(state: State<AppState>) -> String {
    match get_files_info_with_error(state) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn get_tag_info(state: State<AppState>) -> String {
    match get_tags_info_with_error(state) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn get_init_status(state: State<AppState>) -> String {
    match get_init_status_with_error(state) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn cancel_init_task(state: State<AppState>) -> String {
    let app_state = state.inner().clone();
    app_state
        .cancel_flag
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let res = BaseResponse {
        response_code: 0,
        response_msg: 0,
    };
    serde_json::to_string(&res).unwrap_or_default()
}
#[tauri::command]
pub fn set_language(state: State<AppState>, language: String) -> String {
    match set_language_with_error(state, language) {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
#[tauri::command]
pub fn init_git_async(state: State<AppState>, repo_path: String) -> String {
    let sql_lite = state.inner().clone();
    let repo = Repository::open(repo_path.clone());
    if repo.is_err() {
        let res = BaseResponse {
            response_code: 1,
            response_msg: "Please select a valid git repository".to_string(),
        };
        return serde_json::to_string(&res).unwrap();
    };
    thread::spawn({
        move || {
            if let Err(e) = init_git_with_error(sql_lite, repo_path) {
                error!("error: {}", e);
            }
        }
    });
    let res = BaseResponse {
        response_code: 0,
        response_msg: 0,
    };
    serde_json::to_string(&res).unwrap()
}

#[tauri::command]
pub fn get_about_version() -> String {
    match get_about_version_with_error() {
        Ok(item) => {
            let res = BaseResponse {
                response_code: 0,
                response_msg: item,
            };
            serde_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res = BaseResponse {
                response_code: 1,
                response_msg: e.to_string(),
            };
            serde_json::to_string(&res).unwrap()
        }
    }
}
