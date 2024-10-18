use std::collections::HashMap;

use crate::{
    sql_lite::connection::AppState,
    vojo::{author_of_month_response::AuthorOfMonthResponse, git_statistic::GitStatisticInfo},
};
use rusqlite::{params, Connection};
use tauri::State;

pub fn get_authors_info_with_error(
    state: State<AppState>,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT quota_name,quota_value FROM git_author_info")?;
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
pub fn save_author_info(
    git_statistic_info: GitStatisticInfo,
    connections: &Connection,
) -> Result<(), anyhow::Error> {
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let mut total_authors_statistic_info_list: Vec<_> = git_statistic_info_cloned
            .author_statistic_info
            .total_author_statistic_info
            .total_authors
            .values()
            .clone()
            .collect();
        total_authors_statistic_info_list.sort_by(|a, b| b.total_commit.cmp(&a.total_commit));
        let len = total_authors_statistic_info_list.len().min(20);
        total_authors_statistic_info_list = total_authors_statistic_info_list
            .into_iter()
            .take(len)
            .collect();
        let total_authors_statistic_info_list_value =
            serde_json::to_string(&total_authors_statistic_info_list)?;
        connections.execute(
            "insert into git_author_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "total_authors_statistic_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let authors_map = git_statistic_info_cloned
            .author_statistic_info
            .author_of_month_statistic_info
            .authors_map;
        let author_of_month_response = AuthorOfMonthResponse::from_hashmap(authors_map);
        let total_authors_statistic_info_list_value =
            serde_json::to_string(&author_of_month_response)?;
        connections.execute(
            "insert into git_author_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "author_of_month_statistic_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let authors_map = git_statistic_info_cloned
            .author_statistic_info
            .author_of_year_statistic_info
            .authors_map;
        let author_of_month_response = AuthorOfMonthResponse::from_hashmap(authors_map);

        let total_authors_statistic_info_list_value =
            serde_json::to_string(&author_of_month_response)?;
        connections.execute(
            "insert into git_author_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "author_of_year_statistic_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }

    Ok(())
}
