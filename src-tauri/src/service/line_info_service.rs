use std::collections::HashMap;

use crate::{
    sql_lite::connection::AppState,
    vojo::git_statistic::{GitStatisticInfo, LineStatisticInfoItem},
};
use chrono::{Duration, NaiveDateTime};
use rusqlite::{params, Connection};
use serde::Serialize;
use tauri::State;
pub fn get_line_info_with_error(
    state: State<AppState>,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT quota_name,quota_value FROM git_line_info")?;
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
#[derive(Serialize)]
pub struct DirLocInfoItem {
    pub dir_name: String,
    pub data: Vec<(String, i32)>,
}
pub fn save_line_info(
    git_statistic_info: GitStatisticInfo,
    connections: &Connection,
) -> Result<(), anyhow::Error> {
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let total_lines = git_statistic_info_cloned.line_statistic_info.total_lines;
        let total_lines_string = total_lines.to_string();
        connections.execute(
            "insert into git_line_info (quota_name,quota_value)
    values (?1,?2)",
            params!["line_statistic_total_count", total_lines_string],
        )?;
    }
    let mut start_time = "".to_string();
    let mut end_time = "".to_string();
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let mut line_statistic_map = git_statistic_info_cloned
            .line_statistic_info
            .line_statistic_base_info;
        let sorted_vec = map_to_list(&mut line_statistic_map, None, None)?;
        start_time = sorted_vec.first().ok_or(anyhow!(""))?.date.clone();
        end_time = sorted_vec.last().ok_or(anyhow!(""))?.date.clone();
        let lines_count_data = serde_json::to_string(&sorted_vec)?;
        connections.execute(
            "insert into git_line_info (quota_name,quota_value)
    values (?1,?2)",
            params!["line_statistic_data", lines_count_data],
        )?;
    };
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let mut line_statistic_map = git_statistic_info_cloned
            .line_statistic_info
            .directory_loc_info;
        let mut dirs_loc_list = vec![];
        for (key, value) in line_statistic_map.iter_mut() {
            // info!("key is {},value is {:?}", key, value);
            let sorted_vec = map_to_list(value, Some(start_time.clone()), Some(end_time.clone()))?;
            dirs_loc_list.push((key, sorted_vec));
        }
        dirs_loc_list.sort_by(|a, b| a.0.cmp(b.0));
        let dirs: Vec<DirLocInfoItem> = dirs_loc_list
            .into_iter()
            .map(|(key, value)| {
                let data: Vec<(String, i32)> = value
                    .into_iter()
                    .map(|item| (item.date, item.count))
                    .collect();
                DirLocInfoItem {
                    dir_name: key.clone(),
                    data,
                }
            })
            .collect();
        let lines_count_data = serde_json::to_string(&dirs)?;
        connections.execute(
            "insert into git_line_info (quota_name,quota_value)
    values (?1,?2)",
            params!["dir_loc_info", lines_count_data],
        )?;
        {
            let init_dirs = serde_json::to_string(&vec!["/"])?;
            connections.execute(
                "insert into git_line_info (quota_name,quota_value)
    values (?1,?2)",
                params!["dirs_for_line_info", init_dirs],
            )?;
        }
    };

    Ok(())
}
fn map_to_list(
    line_statistic_map: &mut HashMap<String, LineStatisticInfoItem>,
    start_time_option: Option<String>,
    end_time_option: Option<String>,
) -> Result<Vec<LineStatisticInfoItem>, anyhow::Error> {
    let (start_time, end_time) = if let (Some(a), Some(b)) = (start_time_option, end_time_option) {
        (a, b)
    } else {
        let format = "%Y-%m-%d %H:%M:%S";
        let mut start_time = NaiveDateTime::parse_from_str(
            line_statistic_map
                .keys()
                .next()
                .ok_or(anyhow!(""))?
                .clone()
                .as_str(),
            format,
        )?;
        let mut end_time = start_time;

        for date_str in line_statistic_map.keys() {
            if let Ok(date) = NaiveDateTime::parse_from_str(date_str, format) {
                if date < start_time {
                    start_time = date;
                }
                if date > end_time {
                    end_time = date;
                }
            }
        }
        start_time -= Duration::days(1);
        end_time += Duration::days(1);
        (
            start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        )
    };

    let date_strings = {
        let start_date = NaiveDateTime::parse_from_str(&start_time, "%Y-%m-%d %H:%M:%S")?;
        let end_date = NaiveDateTime::parse_from_str(&end_time, "%Y-%m-%d %H:%M:%S")?;
        let mut current_date = start_date;
        let mut date_strings = Vec::new();
        while current_date <= end_date {
            date_strings.push(current_date.format("%Y-%m-%d %H:%M:%S").to_string());
            current_date += Duration::days(1);
        }
        date_strings
    };
    for date_string in date_strings {
        line_statistic_map
            .entry(date_string.clone())
            .or_insert(LineStatisticInfoItem {
                count: 0,
                date: date_string,
            });
    }
    let mut list = line_statistic_map.iter_mut().collect::<Vec<_>>();
    list.sort_by(|a, b| a.0.cmp(b.0));
    let mut sorted_vec: Vec<LineStatisticInfoItem> =
        list.into_iter().map(|(_, value)| value.clone()).collect();
    for i in 1..sorted_vec.len() {
        sorted_vec[i].count += sorted_vec[i - 1].count;
    }
    Ok(sorted_vec)
}
pub fn save_dirs_for_line_info_with_error(
    state: State<AppState>,
    dirs: Vec<String>,
) -> Result<(), anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let dirs_str = serde_json::to_string(&dirs)?;
    connection.execute(
        "REPLACE INTO git_line_info (quota_name, quota_value) VALUES (?1, ?2)",
        params!["dirs_for_line_info", dirs_str],
    )?;
    Ok(())
}
