use std::collections::HashMap;

use crate::{sql_lite::connection::AppState, vojo::git_statistic::GitStatisticInfo};
use rusqlite::{params, Connection};
use tauri::State;
pub fn get_files_info_with_error(
    state: State<AppState>,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT quota_name,quota_value FROM git_file_info")?;
    let rows: Vec<_> = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect();
    let mut hash_map = HashMap::new();
    for item in rows {
        let (key, value) = item?;
        hash_map.insert(key, value);
    }

    Ok(hash_map)
}
pub fn save_files_info(
    git_statistic_info: GitStatisticInfo,
    connections: &Connection,
) -> Result<(), anyhow::Error> {
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let file_statistic_base_info = git_statistic_info_cloned
            .file_statistic_info
            .file_statistic_base_info;
        let total_authors_statistic_info_list_value =
            serde_json::to_string(&file_statistic_base_info)?;
        connections.execute(
            "insert into git_file_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "files_statistic_base_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let file_statistic_extension_info = git_statistic_info_cloned
            .file_statistic_info
            .file_statistic_extension_info;
        let total_authors_statistic_info_list_value =
            serde_json::to_string(&file_statistic_extension_info)?;
        connections.execute(
            "insert into git_file_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "files_statistic_ext_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }

    Ok(())
}
