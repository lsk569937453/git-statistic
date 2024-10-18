use crate::sql_lite::connection::AppState;
use crate::vojo::git_statistic::*;
use tauri::State;

pub fn get_base_info_with_error(state: State<AppState>) -> Result<GitBaseInfo, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT age,project_name,generate_time,active_days,total_files_count,total_lines_count,total_added_count,total_deleted_count,total_commits_count,authors_count,first_commit_time,last_commit_time FROM git_base_info")?;
    let rows: Vec<_> = statement
        .query_map([], |row| {
            Ok(GitBaseInfo {
                age: row.get(0)?,
                project_name: row.get(1)?,
                generate_time: row.get(2)?,
                active_days: row.get(3)?,
                total_files: row.get(4)?,
                total_lines: row.get(5)?,
                total_added: row.get(6)?,
                total_deleted: row.get(7)?,
                total_commits: row.get(8)?,
                authors: row.get(9)?,
                first_commit_time: row.get(10)?,
                last_commit_time: row.get(11)?,
            })
        })?
        .collect();
    let git_base_info = rows.into_iter().next().ok_or(anyhow!(""))??;

    Ok(git_base_info)
}
