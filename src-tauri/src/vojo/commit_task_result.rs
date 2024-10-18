use std::collections::HashMap;

use chrono::{DateTime, Local};
//DateTime<Local>, String, i32, i32, bool
pub struct CommitTaskResult {
    pub commit_time: DateTime<Local>,
    pub author_name: String,
    pub added: i32,
    pub deleted: i32,
    pub cal_flag: bool,
    pub file_add_del_map: HashMap<String, (i32, i32)>,
}
impl CommitTaskResult {
    pub fn new(
        commit_time: DateTime<Local>,
        author_name: String,
        added: i32,
        deleted: i32,
        cal_flag: bool,
        file_add_del_map: HashMap<String, (i32, i32)>,
    ) -> Self {
        Self {
            commit_time,
            author_name,
            added,
            deleted,
            cal_flag,
            file_add_del_map,
        }
    }
}
