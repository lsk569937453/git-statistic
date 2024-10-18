use crate::service::sqlite_service::clear_data;
use crate::sql_lite::connection::AppState;
use crate::vojo::commit_task_result::CommitTaskResult;
use crate::vojo::git_statistic::*;
use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use git2::Delta;
use git2::DiffFindOptions;
use git2::DiffFormat;
use git2::Oid;
use git2::{DiffOptions, Repository, TreeWalkMode, TreeWalkResult};
use rayon::prelude::*;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use tauri::State;

use super::authors_service::save_author_info;
use super::commit_info_service::save_commit_info;
use super::file_info_service::save_files_info;
use super::line_info_service::save_line_info;
use super::tags_info_service::save_tag_info;

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
    revwalk.push_head()?;
    // revwalk.simplify_first_parent()?;
    let commit_task_count = revwalk.collect::<Result<Vec<_>, _>>()?.len();
    let mut tag_set = vec![];
    let refs = repo.references()?;
    for r in refs {
        let r = r?;
        if r.shorthand().is_some() {
            if let Some(target) = r.target() {
                if r.is_tag() {
                    tag_set.push(target);
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
    // info!("base info is {}", git_statis_info);
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

fn analyze_base_info(
    state: AppState,
    repo_path: String,
) -> Result<GitStatisticInfo, anyhow::Error> {
    let mut git_statistic_info = GitStatisticInfo::new();
    let repo = Repository::open(repo_path.clone())?;
    let total_files = get_files_count(&repo)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut revwalks = revwalk.collect::<Result<Vec<_>, _>>()?;
    revwalks.reverse();

    let last_commit_oid = revwalks.last().ok_or(anyhow!(""))?;
    let last_commit = repo.find_commit(*last_commit_oid)?;
    let last_commit_time = Utc
        .timestamp_opt(last_commit.time().seconds(), 0)
        .single()
        .ok_or(anyhow!(""))?;

    let mut total_lines_count = 0;
    let oid_hash_set = get_oid_of_main_line(repo_path.clone())?;
    let (mut added_total, mut deleted_total) = (0, 0);

    let task_results = revwalks
        .par_iter()
        .map(|commit| -> Result<CommitTaskResult, anyhow::Error> {
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
            let commitx = *commit;
            let commit = repo
                .find_commit(commitx)
                .map_err(|e| anyhow!("Can not find commit {}", e))?;
            //only calculate the total added,deleted lines for the first parent like --first-parent
            let cal_flag = { oid_hash_set.contains(&commitx) };

            let commit_cloned = commit.clone();
            let first_parent_tree = if let Ok(item) = commit.parent(0) {
                Some(item.tree()?)
            } else {
                None
            };
            let author = commit_cloned.author();
            let author_name = author.name().ok_or(anyhow!("can not find name"))?;
            let b = commit.tree()?;
            let mut diff =
                repo.diff_tree_to_tree(first_parent_tree.as_ref(), Some(&b), Some(&mut diffopts2))?;
            let mut diff_find = DiffFindOptions::new();
            diff.find_similar(Some(&mut diff_find))?;

            let mut added = 0;
            let mut deleted = 0;
            let mut file_add_del_map = HashMap::new();
            diff.print(DiffFormat::Patch, |delta, _hunk, line| {
                let status = delta.status();
                match status {
                    Delta::Added | Delta::Modified | Delta::Deleted => {
                        if let Some(new_file) = delta.new_file().path() {
                            let filename = new_file.display().to_string();
                            let current_added = if line.origin() == '+' { 1 } else { 0 };
                            let current_deleted = if line.origin() == '-' { 1 } else { 0 };

                            added += current_added;
                            deleted += current_deleted;
                            file_add_del_map
                                .entry(filename.clone())
                                .and_modify(|(a, b)| {
                                    *a += current_added;
                                    *b += current_deleted;
                                })
                                .or_insert((current_added, current_deleted));
                        }
                    }
                    _ => (),
                }
                true
            })?;

            let author_name = author_name.to_string();

            let commit_time = Utc
                .timestamp_opt(commit.time().seconds(), 0)
                .single()
                .ok_or(anyhow!(""))?;
            let converted: DateTime<Local> = DateTime::from(commit_time);
            let commit_task_result = CommitTaskResult::new(
                converted,
                author_name,
                added,
                deleted,
                cal_flag,
                file_add_del_map.clone(),
            );

            Ok(commit_task_result)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let total_commits = task_results.len() as i32;
    let mut authors = HashSet::new();
    info!("task_results commits count is {}", task_results.len());
    for item in task_results {
        git_statistic_info.calc_commit(
            item.commit_time,
            item.author_name.to_string(),
            item.added,
            item.deleted,
            item.file_add_del_map,
            item.cal_flag,
        );
        authors.insert(item.author_name.clone());

        if item.cal_flag {
            added_total += item.added;
            deleted_total += item.deleted;
            total_lines_count = total_lines_count + item.added - item.deleted;
        }
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

fn get_oid_of_main_line(repo_path: String) -> Result<HashSet<Oid>, anyhow::Error> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.simplify_first_parent()?;
    let mut hash_set = HashSet::new();
    for item in revwalk {
        let oid = item?;
        hash_set.insert(oid);
    }

    Ok(hash_set)
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
            let size = blob.size();
            let fullpath = entry.name().unwrap_or_default();

            total_size += size;
            total_files += 1;

            let filename = fullpath.split('/').last().unwrap_or_default();
            let ext = if let Some(idx) = filename.rfind('.') {
                if idx == 0 {
                    String::new()
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
        // info!("item is {},date:{}", item.tag_name, item.date_time);
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
