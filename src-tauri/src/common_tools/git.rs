use crate::sql_lite::connection::AppState;
use crate::vojo::author_of_month_response::AuthorOfMonthResponse;
use crate::vojo::git_statistic::*;
use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use git2::Oid;
use git2::{Commit, DiffFormat, DiffOptions, Repository, TreeWalkMode, TreeWalkResult};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use tauri::State;
pub fn get_base_info_with_error(state: State<AppState>) -> Result<GitBaseInfo, anyhow::Error> {
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    let mut statement = connection.prepare("SELECT age,project_name,generate_time,active_days,total_files_count,total_lines_count,total_added_count,total_deleted_count,total_commits_count,authors_count FROM git_base_info")?;
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
    let sql_lite = state.pool.get()?;
    let connection = &sql_lite;
    clear_data(connection)?;
    init_git_tasks(connection, repo_path.clone())?;
    init_statistic_info(connection, repo_path.clone())?;
    Ok(())
}
fn init_git_tasks(connection: &Connection, repo_path: String) -> Result<(), anyhow::Error> {
    let repo = Repository::open(repo_path.clone())?;
    let mut revwalk = repo.revwalk()?;
    let revspec = repo.revparse_single("HEAD")?.id();
    revwalk.push(revspec)?;
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
fn clear_data(connection: &Connection) -> Result<(), anyhow::Error> {
    connection.execute_batch(
        "DROP TABLE IF EXISTS git_base_info;
            DROP TABLE IF EXISTS git_commit_info;
            DROP TABLE IF EXISTS git_author_info;
            DROP TABLE IF EXISTS git_file_info;
            DROP TABLE IF EXISTS git_tag_info;
            DROP TABLE IF EXISTS git_init_status;

        ",
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_init_status (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            current_tasks INTEGER NOT NULL, 
            total_tasks INTEGER NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_base_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            project_name TEXT NOT NULL, 
            generate_time TEXT NOT NULL,
            age    INTEGER NOT NULL, 
            active_days  INTEGER NOT NULL,
            total_files_count INTEGER NOT NULL,
            total_lines_count INTEGER NOT NULL,
            total_added_count INTEGER NOT NULL,
            total_deleted_count INTEGER NOT NULL,
            total_commits_count INTEGER NOT NULL,
            authors_count INTEGER NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_commit_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_author_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_file_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_tag_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;

    Ok(())
}
fn init_statistic_info(connections: &Connection, repo: String) -> Result<(), anyhow::Error> {
    let git_statis_info = analyze_base_info(connections, repo)?;
    info!("base info is {}", git_statis_info);
    let base_info = git_statis_info.clone().git_base_info;
    connections.execute(
        "insert into git_base_info (age,project_name,generate_time,active_days,total_files_count,total_lines_count,total_added_count,total_deleted_count,total_commits_count,authors_count) 
        values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
        params![base_info.age,
        base_info.project_name,
        base_info.generate_time,
            base_info.active_days,
            base_info.total_files,
            base_info.total_lines,
            base_info.total_added,
            base_info.total_deleted,
            base_info.total_commits,
            base_info.authors],
    )?;
    save_commit_info(git_statis_info.clone(), connections)?;
    save_author_info(git_statis_info.clone(), connections)?;
    save_files_info(git_statis_info.clone(), connections)?;
    save_tag_info(git_statis_info, connections)?;

    Ok(())
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
fn analyze_base_info(
    connections: &Connection,
    repo_path: String,
) -> Result<GitStatisticInfo, anyhow::Error> {
    let mut git_statistic_info = GitStatisticInfo::new();
    let repo = Repository::open(repo_path.clone())?;

    let total_files = get_files_count(&repo)?;
    let mut revwalk = repo.revwalk()?;
    let revspec = repo.revparse_single("HEAD")?.id();
    revwalk.push(revspec)?;
    let revwalks = revwalk.collect::<Result<Vec<_>, _>>()?;

    let commit = repo.find_commit(*revwalks.first().ok_or(anyhow!(""))?)?;
    let total_lines_count = get_lines_count(commit.clone(), &repo)?.1;

    let (mut added_total, mut deleted_total) = (0, 0);

    let (_, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());

    let mut authors = HashSet::new();
    let mut last_commit_oid = Oid::zero();
    let commit_count = revwalks.len();
    info!("real commit count is {}", commit_count);
    let task_results = revwalks
        .iter()
        .map(
            |commit| -> Result<(DateTime<Local>, String, i32, i32), anyhow::Error> {
                connections.execute(
                    "UPDATE git_init_status SET current_tasks = current_tasks + 1",
                    params![],
                )?;
                let (mut added, mut deleted) = (0, 0);
                let commitx = *commit;
                last_commit_oid = commitx;
                let commit = repo.find_commit(commitx)?;

                let commit_cloned = commit.clone();

                let a = if commit.parents().len() == 1 {
                    let parent = commit.parent(0)?;
                    Some(parent.tree()?)
                } else {
                    None
                };

                let author = commit_cloned.author();
                let author_name = author.name().ok_or(anyhow!("can not find name"))?;
                authors.insert(author_name.to_string());

                if a.is_some() {
                    let b = commit.tree()?;
                    let diff =
                        repo.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diffopts2))?;
                    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
                        if line.origin() == '+' {
                            added += 1;
                        } else if line.origin() == '-' {
                            deleted += 1;
                        }
                        true
                    })?;
                    added_total += added;
                    deleted_total += deleted;
                }
                let commit_time = Utc
                    .timestamp_opt(commit.time().seconds(), 0)
                    .single()
                    .ok_or(anyhow!(""))?;
                let converted: DateTime<Local> = DateTime::from(commit_time);
                // git_statistic_info.calc_commit(converted, author_name.to_string(), added, deleted);
                Ok((converted, author_name.to_string(), added, deleted))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let total_commits = task_results.len() as i32;
    for (converted, author_name, added, deleted) in task_results {
        git_statistic_info.calc_commit(converted, author_name.to_string(), added, deleted);
    }

    let last_commit = repo.find_commit(last_commit_oid)?;
    let first_commit_time = Utc
        .timestamp_opt(last_commit.time().seconds(), 0)
        .single()
        .ok_or(anyhow!(""))?;

    info!("first commit time is {}", first_commit_time);

    git_statistic_info.file_statistic_info = analyze_files(&repo)?;

    git_statistic_info.tag_statistic_info = analyze_tag(connections, &repo)?;
    let now = Utc::now();
    let age = now.signed_duration_since(first_commit_time);

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
        age: age.num_days() as i32,
        active_days: age.num_weeks() as i32,
        total_files,
        total_lines: total_lines_count,
        total_added: added_total,
        total_deleted: deleted_total,
        total_commits,
        authors: authors.len() as i32,
    };

    Ok(git_statistic_info)
}
fn analyze_files(repo: &Repository) -> Result<FileStatisticInfo, anyhow::Error> {
    let head_commit = repo.head()?.peel_to_commit()?;
    let tree = head_commit.tree()?;
    info!("xxxt");

    let mut total_size = 0;
    let mut total_files = 0;
    let mut total_lines = 0;

    let mut file_statistic_ext_map = HashMap::new();
    tree.walk(TreeWalkMode::PreOrder, |_, entry| {
        if let Some(blob) = entry
            .to_object(repo)
            .ok()
            .and_then(|o| o.as_blob().cloned())
        {
            let blob_id = blob.id().to_string();
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
            total_lines += line_count as i32;
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
fn analyze_tag(
    connections: &Connection,
    repo: &Repository,
) -> Result<TagStatisticInfo, anyhow::Error> {
    let refs = repo.references()?;
    let mut map = HashMap::new();
    let mut tag_refs: Vec<(Oid, String)> = vec![];
    for r in refs {
        let r = r?;
        if let Some(name) = r.shorthand() {
            if let Some(target) = r.target() {
                // Filter tags
                if r.is_tag() {
                    tag_refs.push((target, name.to_string()));
                }
            }
        }
    }

    // Process each tag
    for (hash, tag) in tag_refs {
        let time = if let Ok(commit) = repo
            .find_commit(hash)
            .map_err(|e| anyhow!("Can not find commit {}", e))
        {
            let commit = repo
                .find_commit(hash)
                .map_err(|e| anyhow!("Can not find commit {}", e))?;

            Utc.timestamp_opt(commit.time().seconds(), 0)
                .single()
                .ok_or(anyhow!(""))?
        } else if let Ok(tag_obj) = repo.find_tag(hash) {
            Utc.timestamp_opt(
                tag_obj
                    .tagger()
                    .ok_or(anyhow!("Invalid tag time"))?
                    .when()
                    .seconds(),
                0,
            )
            .single()
            .ok_or(anyhow!("Invalid tag time"))?
        } else {
            Err(anyhow!(""))?
        };
        let converted: DateTime<Local> = DateTime::from(time);
        let year_and_month = converted.format("%Y-%m-%d").to_string();

        map.insert(
            hash,
            TagStatisticMainInfoItem::new(tag.clone(), year_and_month, 0, vec![]),
        );
    }
    let mut prev: Option<Oid> = None;
    let mut total_commit = 0;
    let tag_count = map.len() as i32;
    info!("real tag count is {}", tag_count);
    for (tag_oid, tag_info) in map.iter_mut() {
        connections.execute(
            "UPDATE git_init_status SET current_tasks = current_tasks + 1",
            params![],
        )?;
        if let Ok(r) = repo.find_commit(*tag_oid) {
            // Create a hashmap to count commits by author for this tag
            let mut author_count: HashMap<String, usize> = HashMap::new();
            let mut commit_count = 0;

            // Get the commit history for this tag
            let mut revwalk = repo.revwalk()?;
            revwalk.push(*tag_oid)?;

            for oid in revwalk {
                let commit_oid = oid?;
                if let Some(prev_oid) = prev {
                    if commit_oid == prev_oid {
                        break; // Stop if we reach the previous tag
                    }
                }
                let commit = repo.find_commit(commit_oid)?;
                let author_name = commit.author().name().unwrap_or("Unknown").to_string();
                // Increment commit count and add to author count
                commit_count += 1;
                total_commit += 1;
                *author_count.entry(author_name).or_insert(0) += 1;
            }
            tag_info.commit_count = commit_count;
            tag_info.authors = author_count.into_iter().collect();

            prev = Some(*tag_oid);
        }
    }

    let total_tags = map.len() as i32;
    let val = total_commit as f64 / total_tags as f64;
    let average_commit_per_tag = format!("{:.2}", val);
    let tag_statistic_base_info = TagStatisticBaseInfo {
        total_tags,
        average_commit_per_tag,
    };
    let mut tag_statistic_main_info_list = map
        .values()
        .cloned()
        .collect::<Vec<TagStatisticMainInfoItem>>();
    tag_statistic_main_info_list.sort_by(|a, b| b.date.cmp(&a.date));
    for item in tag_statistic_main_info_list.iter_mut() {
        // Convert HashMap to a Vec of tuples, then sort by the value (usize)
        let mut sorted_authors: Vec<(String, usize)> = item.authors.clone().into_iter().collect();
        sorted_authors.sort_by(|a, b| b.1.cmp(&a.1)); // Sort in descending order

        // Replace the original authors map with the sorted vector converted back into a HashMap
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
fn get_lines_count(commit: Commit, repo: &Repository) -> Result<(i32, i32), anyhow::Error> {
    let tree = commit.tree()?;
    let (mut total_files, mut total_lines) = (0, 0);

    let _ = tree.walk(TreeWalkMode::PreOrder, |_, entry| {
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let obj = entry.to_object(repo).unwrap();
            let blob = obj.as_blob().ok_or(anyhow!("erros ")).unwrap();
            let file_lines = blob.content().split(|&c| c == b'\n').count();
            total_lines += file_lines as i32;
            total_files += 1;
        }
        TreeWalkResult::Ok
    });

    info!(
        "Total lines of code in commit: {},total files count:{}",
        total_lines, total_files
    );
    Ok((total_files, total_lines))
}
