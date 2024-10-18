use std::collections::HashMap;

use crate::{sql_lite::connection::AppState, vojo::git_statistic::GitStatisticInfo};
use rusqlite::{params, Connection};
use tauri::State;
pub fn get_commit_info_with_error(
    state: State<AppState>,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT quota_name,quota_value FROM git_commit_info")?;
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
pub fn save_commit_info(
    git_statistic_info: GitStatisticInfo,
    connections: &Connection,
) -> Result<(), anyhow::Error> {
    {
        let recent_weeks_commit = git_statistic_info.clone().commit_info.recent_weeks_commit;
        let recent_weeks_commit_value = serde_json::to_string(&recent_weeks_commit)?;
        connections.execute(
            "insert into git_commit_info (quota_name,quota_value)
    values (?1,?2)",
            params!["recent_weeks_commit", recent_weeks_commit_value],
        )?;
    }
    {
        let hours_of_day_commit = git_statistic_info.clone().commit_info.hours_commit;
        let recent_weeks_commit_value = serde_json::to_string(&hours_of_day_commit)?;
        connections.execute(
            "insert into git_commit_info (quota_name,quota_value)
    values (?1,?2)",
            params!["hours_of_day_commit", recent_weeks_commit_value],
        )?;
    }
    {
        let day_of_week = git_statistic_info.clone().commit_info.day_of_week_commit;
        let recent_weeks_commit_value = serde_json::to_string(&day_of_week)?;
        connections.execute(
            "insert into git_commit_info (quota_name,quota_value)
    values (?1,?2)",
            params!["day_of_week", recent_weeks_commit_value],
        )?;
    }
    {
        let month_of_year_commit = git_statistic_info.clone().commit_info.month_of_year_commit;
        let recent_weeks_commit_value = serde_json::to_string(&month_of_year_commit)?;
        connections.execute(
            "insert into git_commit_info (quota_name,quota_value)
    values (?1,?2)",
            params!["month_of_year_commit", recent_weeks_commit_value],
        )?;
    }
    {
        let year_and_month_commit = git_statistic_info.clone().commit_info.year_and_month_commit;
        let recent_weeks_commit_value = serde_json::to_string(&year_and_month_commit)?;
        connections.execute(
            "insert into git_commit_info (quota_name,quota_value)
    values (?1,?2)",
            params!["year_and_month_commit", recent_weeks_commit_value],
        )?;
    }
    {
        let year_commit = git_statistic_info.clone().commit_info.year_commit;
        let recent_weeks_commit_value = serde_json::to_string(&year_commit)?;
        connections.execute(
            "insert into git_commit_info (quota_name,quota_value)
    values (?1,?2)",
            params!["year_commit", recent_weeks_commit_value],
        )?;
    }
    Ok(())
}
