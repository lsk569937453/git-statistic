use crate::sql_lite::connection::{SqlLite, SqlLiteState};
use crate::vojo::git_statistic::*;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use git2::Oid;
use git2::{
    Commit, DiffFormat, DiffOptions, Error, ErrorCode, ObjectType, Repository, StatusOptions,
    SubmoduleIgnore, Time, Tree, TreeWalkMode, TreeWalkResult,
};
use rusqlite::{params, Connection};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use tauri::State;
pub fn get_base_info_with_error(state: State<SqlLiteState>) -> Result<GitBaseInfo, anyhow::Error> {
    let sql_lite = state.0.lock().map_err(|e| anyhow!("lock error"))?;
    let connection = &sql_lite.connection;
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
fn get_files_count(repo: Repository) -> Result<i32, anyhow::Error> {
    let index = repo.index()?;
    let mut current_lines_count = 0;
    for entry in index.iter() {
        let path = entry.path;
        current_lines_count += 1;
    }
    Ok(current_lines_count)
}
pub fn init_git_with_error(
    state: State<SqlLiteState>,
    repo_path: String,
) -> Result<(), anyhow::Error> {
    info!("repo path is {}", repo_path);
    // let repo = git2::Repository::open(repo_path)?;
    let sql_lite = state.0.lock().map_err(|e| anyhow!("lock error"))?;

    let connection = &sql_lite.connection;
    save_base_info(connection, repo_path.clone())?;
    Ok(())
}
fn save_base_info(connections: &Connection, repo: String) -> Result<(), anyhow::Error> {
    let git_statis_info = analyze_base_info(repo)?;
    info!("base info is {:?}", git_statis_info);
    let base_info = git_statis_info.git_base_info;
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
    Ok(())
}
fn analyze_base_info(repo_path: String) -> Result<GitStatisticInfo, anyhow::Error> {
    let mut git_statistic_info = GitStatisticInfo::new();
    let new_repo = Repository::open(repo_path.clone())?;
    let repo = Repository::open(repo_path.clone())?;

    let total_files = get_files_count(new_repo)?;
    let mut revwalk = repo.revwalk()?;
    let revspec = repo.revparse_single("HEAD")?.id();
    revwalk.push(revspec)?;

    let mut total_commits = 0;
    let (mut added_total, mut deleted_total) = (0, 0);

    let (mut diffopts, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());

    let mut total_lines_count = 0;
    let mut authors = HashSet::new();
    let mut last_commit_oid = Oid::zero();
    for commit in revwalk {
        let (mut added, mut deleted) = (0, 0);
        let commitx = commit?;
        last_commit_oid = commitx;
        let commit = repo.find_commit(commitx)?;

        let commit_cloned = commit.clone();

        let a = if commit.parents().len() == 1 {
            let parent = commit.parent(0)?;
            Some(parent.tree()?)
        } else {
            None
        };
        if a.is_none() {
            continue;
        }
        let commit_time = Utc
            .timestamp_opt(commit.time().seconds(), 0)
            .single()
            .ok_or(anyhow!(""))?;
        git_statistic_info.calc(commit_time);
        let author = commit_cloned.author();
        let author_name = author.name().ok_or(anyhow!("can not find name"))?;
        authors.insert(author_name.to_string());

        let b = commit.tree()?;
        let diff = repo.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diffopts2))?;
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            if line.origin() == '+' {
                added += 1;
            } else if line.origin() == '-' {
                deleted += 1;
            }
            true
        })?;
        let repo2 = Repository::open(repo_path.clone())?;
        if total_commits == 0 {
            total_lines_count = get_lines_count(commit, repo2)?;
        }
        added_total += added;
        deleted_total += deleted;
        total_commits += 1;
    }
    let last_commit = repo.find_commit(last_commit_oid)?;
    let first_commit_time = Utc
        .timestamp_opt(last_commit.time().seconds(), 0)
        .single()
        .ok_or(anyhow!(""))?;

    info!("first commit time is {}", first_commit_time);
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
fn get_lines_count(commit: Commit, repo: Repository) -> Result<i32, anyhow::Error> {
    let tree = commit.tree()?;
    let mut total_lines = 0;
    let _ = tree.walk(TreeWalkMode::PreOrder, |_, entry| {
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let obj = entry.to_object(&repo).unwrap();
            let blob = obj.as_blob().ok_or(anyhow!("erros ")).unwrap();
            let file_lines = blob.content().split(|&c| c == b'\n').count();
            total_lines += file_lines as i32;
        }
        TreeWalkResult::Ok
    });

    println!("Total lines of code in commit: {}", total_lines);
    Ok(total_lines)
}
