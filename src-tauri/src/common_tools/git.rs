use crate::service::sqlite_service::clear_data;
use crate::sql_lite::connection::AppState;
use crate::vojo::author_of_month_response::AuthorOfMonthResponse;
use crate::vojo::git_statistic::*;
use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use git2::Oid;
use git2::{DiffOptions, Repository, TreeWalkMode, TreeWalkResult};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
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
pub fn get_tags_info_with_error(
    state: State<AppState>,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT quota_name,quota_value FROM git_tag_info")?;
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
pub fn get_init_status_with_error(state: State<AppState>) -> Result<(i32, i32), anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement =
        connection.prepare("SELECT current_tasks,total_tasks FROM git_init_status")?;
    let rows: Vec<_> = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect();
    let mut data = (0, 0);
    for item in rows {
        data = item?;
    }

    Ok(data)
}
fn save_commit_info(
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
fn save_author_info(
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
fn save_files_info(
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
fn save_line_info(
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

        let git_statistic_info_cloned = git_statistic_info.clone();
        let mut line_statistic_map = git_statistic_info_cloned
            .line_statistic_info
            .line_statistic_base_info;
        let (start_time, end_time) = {
            let format = "%Y-%m-%d %H:%M:%S";
            let mut start_time = line_statistic_map.keys().next().ok_or(anyhow!(""))?.clone();
            let mut end_time = start_time.clone();

            for date_str in line_statistic_map.keys() {
                if let Ok(date) = NaiveDateTime::parse_from_str(date_str, format) {
                    if date < NaiveDateTime::parse_from_str(&start_time, format)? {
                        start_time = date_str.clone();
                    }
                    if date > NaiveDateTime::parse_from_str(&end_time, format)? {
                        end_time = date_str.clone();
                    }
                }
            }
            (start_time, end_time)
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
        let mut list = line_statistic_map.into_iter().collect::<Vec<_>>();
        list.sort_by(|a, b| a.0.cmp(&b.0));
        let mut sorted_vec: Vec<LineStatisticInfoItem> =
            list.into_iter().map(|(_, value)| value).collect();
        for i in 1..sorted_vec.len() {
            sorted_vec[i].count += sorted_vec[i - 1].count;
        }
        let lines_count_data = serde_json::to_string(&sorted_vec)?;
        // Extract the sorted values
        connections.execute(
            "insert into git_line_info (quota_name,quota_value)
    values (?1,?2)",
            params!["line_statistic_data", lines_count_data],
        )?;
    };

    Ok(())
}
fn save_tag_info(
    git_statistic_info: GitStatisticInfo,
    connections: &Connection,
) -> Result<(), anyhow::Error> {
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let file_statistic_base_info = git_statistic_info_cloned
            .tag_statistic_info
            .tag_statistic_base_info;
        let total_authors_statistic_info_list_value =
            serde_json::to_string(&file_statistic_base_info)?;
        connections.execute(
            "insert into git_tag_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "tag_statistic_base_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }
    {
        let git_statistic_info_cloned = git_statistic_info.clone();
        let file_statistic_extension_info = git_statistic_info_cloned
            .tag_statistic_info
            .tag_statistic_main_info;
        let total_authors_statistic_info_list_value =
            serde_json::to_string(&file_statistic_extension_info)?;
        connections.execute(
            "insert into git_tag_info (quota_name,quota_value)
    values (?1,?2)",
            params![
                "tag_statistic_ext_info",
                total_authors_statistic_info_list_value
            ],
        )?;
    }

    Ok(())
}
fn get_files_count(repo: &Repository) -> Result<i32, anyhow::Error> {
    let index = repo.index()?;
    let mut current_lines_count = 0;
    for _ in index.iter() {
        current_lines_count += 1;
    }
    Ok(current_lines_count)
}

pub fn init_git_with_error(state: AppState, repo_path: String) -> Result<(), anyhow::Error> {
    info!("repo path is {}", repo_path);
    let sql_lite = state.pool.clone().get()?;
    let connection = &sql_lite;
    clear_data(connection)?;
    init_git_tasks(connection, repo_path.clone())?;
    init_statistic_info(state, repo_path.clone())?;
    Ok(())
}
fn init_git_tasks(connection: &Connection, repo_path: String) -> Result<(), anyhow::Error> {
    let repo = Repository::open(repo_path.clone())?;
    let mut revwalk = repo.revwalk()?;
    let head_ref = repo
        .head()?
        .resolve()?
        .target()
        .expect("HEAD has no target");

    revwalk.push(head_ref)?;
    let commit_task_count = revwalk.collect::<Result<Vec<_>, _>>()?.len();
    let mut tag_set = HashSet::new();
    let refs = repo.references()?;
    for r in refs {
        let r = r?;
        if r.shorthand().is_some() {
            if let Some(target) = r.target() {
                // Filter tags
                if r.is_tag() {
                    tag_set.insert(target);
                }
            }
        }
    }
    let tag_task_count = tag_set.len();
    info!(
        "tag task count is {},commit task count is {}",
        tag_task_count, commit_task_count
    );
    connection.execute(
        "insert into git_init_status (current_tasks,total_tasks)
    values (?1,?2)",
        params![0, commit_task_count + tag_task_count],
    )?;
    Ok(())
}

fn init_statistic_info(state: AppState, repo: String) -> Result<(), anyhow::Error> {
    let git_statis_info = analyze_base_info(state.clone(), repo)?;
    let connections = state.pool.get()?;
    info!("base info is {}", git_statis_info);
    let base_info = git_statis_info.clone().git_base_info;
    connections.execute(
        "insert into git_base_info (age,project_name,generate_time,active_days,total_files_count,total_lines_count,total_added_count,total_deleted_count,total_commits_count,authors_count,
        first_commit_time,last_commit_time) 
        values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
        params![base_info.age,
        base_info.project_name,
        base_info.generate_time,
            base_info.active_days,
            base_info.total_files,
            base_info.total_lines,
            base_info.total_added,
            base_info.total_deleted,
            base_info.total_commits,
            base_info.authors,
            base_info.first_commit_time,
            base_info.last_commit_time],
    )?;
    save_commit_info(git_statis_info.clone(), &connections)?;
    save_author_info(git_statis_info.clone(), &connections)?;
    save_files_info(git_statis_info.clone(), &connections)?;
    save_line_info(git_statis_info.clone(), &connections)?;

    save_tag_info(git_statis_info, &connections)?;

    Ok(())
}

use rayon::prelude::*;
fn analyze_base_info(
    state: AppState,
    repo_path: String,
) -> Result<GitStatisticInfo, anyhow::Error> {
    let mut git_statistic_info = GitStatisticInfo::new();
    let repo = Repository::open(repo_path.clone())?;

    let total_files = get_files_count(&repo)?;
    let mut revwalk = repo.revwalk()?;
    let head_ref = repo
        .head()?
        .resolve()?
        .target()
        .expect("HEAD has no target");

    revwalk.push(head_ref)?;
    let mut revwalks = revwalk.collect::<Result<Vec<_>, _>>()?;
    revwalks.reverse();

    let last_commit_oid = revwalks.last().ok_or(anyhow!(""))?;
    let last_commit = repo.find_commit(*last_commit_oid)?;
    let last_commit_time = Utc
        .timestamp_opt(last_commit.time().seconds(), 0)
        .single()
        .ok_or(anyhow!(""))?;

    let mut total_lines_count = 0;

    let (mut added_total, mut deleted_total) = (0, 0);

    let task_results = revwalks
        .par_iter()
        .map(
            |commit| -> Result<(DateTime<Local>, String, i32, i32), anyhow::Error> {
                let repo = Repository::open(repo_path.clone())?;
                let (_, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());

                let connections = state.pool.get()?;
                connections.execute(
                    "UPDATE git_init_status SET current_tasks = current_tasks + 1",
                    params![],
                )?;
                let cancel_flag = state.cancel_flag.clone();
                if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    cancel_flag.store(false, std::sync::atomic::Ordering::SeqCst);
                    return Err(anyhow!("The task has been cancelled."));
                }
                let (mut added, mut deleted) = (0, 0);

                let commitx = *commit;
                let commit = repo
                    .find_commit(commitx)
                    .map_err(|e| anyhow!("Can not find commit {}", e))?;
                let author_name = if commit.clone().parent_count() == 0 {
                    let tree = commit.clone().tree()?;

                    // First commit has no parent, treat all content as new insertions
                    let diff_stats = repo.diff_tree_to_tree(None, Some(&tree), None)?.stats()?;

                    added = diff_stats.insertions() as i32;

                    commit
                        .author()
                        .name()
                        .ok_or(anyhow!("can not find name"))?
                        .to_string()
                } else {
                    let commit_cloned = commit.clone();

                    let a = if commit.parents().len() == 1 {
                        let parent = commit.parent(0)?;
                        Some(parent.tree()?)
                    } else if commit.parents().len() > 1 {
                        let first_parent_oid = commit.parent_id(0)?;
                        let first_parent_commit = repo.find_commit(first_parent_oid)?;
                        Some(first_parent_commit.tree()?)
                    } else {
                        None
                    };

                    let author = commit_cloned.author();
                    let author_name = author.name().ok_or(anyhow!("can not find name"))?;

                    if a.is_some() {
                        diffopts2
                            .force_text(false)
                            .ignore_whitespace_eol(false)
                            .ignore_whitespace_change(false)
                            .ignore_whitespace(false)
                            .include_ignored(false)
                            .include_untracked(false)
                            .patience(false)
                            .minimal(false);
                        let b = commit.tree()?;
                        let diff =
                            repo.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diffopts2))?;
                        let stats = diff.stats()?;

                        added = stats.insertions() as i32;
                        deleted = stats.deletions() as i32;
                    }

                    author_name.to_string()
                };
                let commit_time = Utc
                    .timestamp_opt(commit.time().seconds(), 0)
                    .single()
                    .ok_or(anyhow!(""))?;
                let converted: DateTime<Local> = DateTime::from(commit_time);
                Ok((converted, author_name, added, deleted))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let total_commits = get_commit_count(repo_path.clone())?;
    let mut authors = HashSet::new();
    info!("task_results commits count is {}", task_results.len());
    for (converted, author_name, added, deleted) in task_results {
        git_statistic_info.calc_commit(converted, author_name.to_string(), added, deleted);
        authors.insert(author_name);
        added_total += added;
        deleted_total += deleted;
        total_lines_count = total_lines_count + added - deleted;
    }
    let first_commit_oid = revwalks.first().ok_or(anyhow!(""))?;
    let first_commit = repo.find_commit(*first_commit_oid)?;
    let first_commit_time = Utc
        .timestamp_opt(first_commit.time().seconds(), 0)
        .single()
        .ok_or(anyhow!(""))?;

    info!("first commit time is {}", first_commit_time);

    git_statistic_info.file_statistic_info = analyze_files(&repo, total_lines_count)?;

    git_statistic_info.tag_statistic_info = analyze_tag(state, &repo)?;
    let age = { last_commit_time.timestamp() / 86400 - first_commit_time.timestamp() / 86400 + 1 };

    let project_name = Path::new(&repo_path)
        .file_name()
        .ok_or(anyhow!(""))?
        .to_str()
        .ok_or(anyhow!(""))?
        .to_string();
    let generate_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    git_statistic_info.git_base_info = GitBaseInfo {
        project_name,
        generate_time,
        age: age as i32,
        active_days: age as i32,
        total_files,
        total_lines: total_lines_count,
        total_added: added_total,
        total_deleted: deleted_total,
        total_commits,
        authors: authors.len() as i32,
        first_commit_time: first_commit_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        last_commit_time: last_commit_time.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    Ok(git_statistic_info)
}

fn get_commit_count(repo_path: String) -> Result<i32, anyhow::Error> {
    // Open the repository at the current path
    let repo = Repository::open(repo_path)?;

    // Create a revwalker to traverse commits starting from HEAD
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::NONE)?;
    let count: usize = revwalk.count();

    Ok(count as i32)
}
fn analyze_files(repo: &Repository, total_lines: i32) -> Result<FileStatisticInfo, anyhow::Error> {
    let head_commit = repo.head()?.peel_to_commit()?;
    let tree = head_commit.tree()?;

    let mut total_size = 0;
    let mut total_files = 0;

    let mut file_statistic_ext_map = HashMap::new();
    tree.walk(TreeWalkMode::PreOrder, |_, entry| {
        if let Some(blob) = entry
            .to_object(repo)
            .ok()
            .and_then(|o| o.as_blob().cloned())
        {
            // let blob_id = blob.id().to_string();
            let size = blob.size();
            let fullpath = entry.name().unwrap_or_default();

            total_size += size;
            total_files += 1;

            // Strip directories
            let filename = fullpath.split('/').last().unwrap_or_default();
            let ext = if let Some(idx) = filename.rfind('.') {
                if idx == 0 {
                    String::new() // Handle files like `.gitignore`
                } else {
                    filename[(idx + 1)..].to_string()
                }
            } else {
                String::new()
            };

            // Limit extension length
            let conf_max_ext_length = 10; // Example config value
            let ext = if ext.len() > conf_max_ext_length {
                String::new()
            } else {
                ext
            };
            let line_count = blob.content().split(|&c| c == b'\n').count();

            let data = file_statistic_ext_map
                .entry(ext.clone())
                .or_insert_with(|| FileStatisticExtensionInfoItem::new(ext.clone(), 0, 0));
            data.files_count += 1;
            data.lines_count += line_count as i32;
        }

        TreeWalkResult::Ok
    })?;
    let val = total_size as f64 / total_files as f64;
    let rounded = format!("{:.2}", val);

    let file_statistic_base_info = FileStatisticBaseInfo {
        total_files_count: total_files,
        total_lines_count: total_lines,
        average_file_size: rounded,
    };
    let mut file_statisct_ext = file_statistic_ext_map
        .values()
        .cloned()
        .collect::<Vec<FileStatisticExtensionInfoItem>>();
    file_statisct_ext.sort_by(|a, b| b.files_count.cmp(&a.files_count));
    let file_statisctic_info = FileStatisticInfo {
        file_statistic_base_info,
        file_statistic_extension_info: FileStatisticExtensionInfo {
            list: file_statisct_ext,
        },
    };
    Ok(file_statisctic_info)
}

fn analyze_tag(state: AppState, repo: &Repository) -> Result<TagStatisticInfo, anyhow::Error> {
    #[derive(Clone)]
    pub struct TagItem {
        pub tag_name: String,
        pub tag_oid: Oid,
        pub date_time: DateTime<Utc>,
        pub pre_oid: Option<Oid>,
    }
    let refs = repo.references()?;
    let mut tag_refs = vec![];
    for r in refs {
        let r = r?;
        if let Some(name) = r.shorthand() {
            if let Some(target) = r.target() {
                if r.is_tag() {
                    let tag_time = if let Ok(tag) = repo
                        .find_tag(target)
                        .map_err(|e| anyhow!("Can not find tag {}", e))
                    {
                        tag.tagger().ok_or(anyhow!(""))?.when()
                    } else {
                        let tag = repo
                            .find_commit(target)
                            .map_err(|e| anyhow!("Can not find commit {}", e))?;
                        tag.time()
                    };

                    let date_time = Utc
                        .timestamp_opt(tag_time.seconds(), 0)
                        .single()
                        .ok_or(anyhow!(""))?;
                    // let date_time_str = date_time.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
                    tag_refs.push(TagItem {
                        tag_name: name.to_string(),
                        tag_oid: target,
                        date_time,
                        pre_oid: None,
                    });
                }
            }
        }
    }
    tag_refs.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    let mut map = vec![];
    let mut prev: Option<Oid> = None;

    for item in tag_refs.iter_mut() {
        info!("item is {},date:{}", item.tag_name, item.date_time);
        let tag = &item.tag_name;

        let year_and_month = item
            .date_time
            .clone()
            .format("%Y-%m-%d %H:%M:%S.%3f")
            .to_string();

        item.pre_oid = prev;
        let cloned_item = item.clone();
        map.push((
            TagStatisticMainInfoItem::new(tag.clone(), year_and_month, 0, vec![]),
            cloned_item,
        ));
        prev = Some(item.tag_oid);
    }
    let tag_count = map.len() as i32;
    info!("real tag count is {}", tag_count);
    let repo_path = repo.path();
    let total_commit = map
        .par_iter_mut()
        .map(|(tag_info, tag_item)| -> Result<i32, anyhow::Error> {
            let tag_oid = tag_item.tag_oid;
            let prevs = tag_item.pre_oid;
            let repo = Repository::open(repo_path)?;
            let connections = state.pool.get()?;
            connections.execute(
                "UPDATE git_init_status SET current_tasks = current_tasks + 1",
                params![],
            )?;
            let cancel_flag = state.cancel_flag.clone();
            if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                cancel_flag.store(false, std::sync::atomic::Ordering::SeqCst);
                return Err(anyhow!("The task has been cancelled."));
            }
            let mut commit_count = 0;

            {
                let mut author_count: HashMap<String, usize> = HashMap::new();

                let mut revwalk = repo.revwalk()?;
                revwalk.push(tag_oid)?;

                if let Some(prev_oid) = prevs {
                    revwalk.hide(prev_oid)?;
                }

                for oid in revwalk {
                    let commit_oid = oid?;

                    let commit = repo.find_commit(commit_oid)?;
                    let author_name = commit.author().name().unwrap_or("Unknown").to_string();
                    commit_count += 1;
                    *author_count.entry(author_name).or_insert(0) += 1;
                }
                tag_info.commit_count = commit_count;
                tag_info.authors = author_count.into_iter().collect();
            }
            Ok(commit_count)
        })
        .filter_map(Result::ok) // Filter out errors
        .reduce(|| 0, |acc, count| acc + count);

    let total_tags = map.len() as i32;
    let val = if total_tags == 0 {
        0.0
    } else {
        total_commit as f64 / total_tags as f64
    };
    let average_commit_per_tag = format!("{:.2}", val);
    let tag_statistic_base_info = TagStatisticBaseInfo {
        total_tags,
        average_commit_per_tag,
    };
    let mut tag_statistic_main_info_list = map
        .into_iter()
        .map(|(item, _)| item)
        .collect::<Vec<TagStatisticMainInfoItem>>();
    tag_statistic_main_info_list.sort_by(|a, b| b.date.cmp(&a.date));
    for item in tag_statistic_main_info_list.iter_mut() {
        let mut sorted_authors: Vec<(String, usize)> = item.authors.clone().into_iter().collect();
        sorted_authors.sort_by(|a, b| b.1.cmp(&a.1)); // Sort in descending order

        item.authors = sorted_authors;
    }
    let tag_statustic_info = TagStatisticInfo {
        tag_statistic_base_info,
        tag_statistic_main_info: TagStatisticMainInfo {
            list: tag_statistic_main_info_list,
        },
    };
    Ok(tag_statustic_info)
}
