use std::thread;

use crate::common_tools::about::get_about_version_with_error;
use crate::common_tools::base64::base64_decode_with_error;
use crate::common_tools::base64::base64_encode_of_image_with_error;
use crate::common_tools::base64::base64_encode_with_error;
use crate::common_tools::base64::base64_save_image_with_error;
use crate::common_tools::base_response::BaseResponse;

use crate::common_tools::git::get_base_info_with_error;
use crate::common_tools::git::get_commit_info_with_error;
use crate::common_tools::git::init_git_with_error;
use crate::common_tools::sql_lite::get_menu_config_with_error;
use crate::common_tools::sql_lite::reset_menu_index_with_error;
use crate::common_tools::sql_lite::set_menu_index_with_error;
use crate::common_tools::sql_lite::GetMenuConfigReq;
use crate::sql_lite::connection::AppState;
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
pub fn init_git_async(state: State<AppState>, repo_path: String) -> String {
    let sql_lite = state.inner().clone();
    thread::spawn({
        move || {
            if let Err(e) = init_git_with_error(sql_lite, repo_path) {
                error!("error: {:?}", e);
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
pub fn base64_encode(source_string: String) -> String {
    match base64_encode_with_error(source_string) {
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
pub fn base64_decode(source_string: String) -> String {
    match base64_decode_with_error(source_string) {
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
pub fn base64_encode_of_image(source_string: String) -> String {
    match base64_encode_of_image_with_error(source_string) {
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
pub fn base64_save_image(source_string: String) -> String {
    match base64_save_image_with_error(source_string) {
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
#[tauri::command]
pub fn get_menu_config(
    state: State<AppState>,
    get_menu_config_reqs: Vec<GetMenuConfigReq>,
) -> String {
    match get_menu_config_with_error(state, get_menu_config_reqs) {
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
pub fn reset_menu_index(state: State<AppState>) -> String {
    match reset_menu_index_with_error(state) {
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
pub fn set_menu_index(state: State<AppState>, source_index: i32, dst_menu_index: i32) -> String {
    match set_menu_index_with_error(state, source_index, dst_menu_index) {
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
