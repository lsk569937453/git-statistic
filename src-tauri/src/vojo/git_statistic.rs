use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Timelike;
use chrono::Utc;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitBaseInfo {
    pub project_name: String,
    pub generate_time: String,
    pub age: i32,
    pub active_days: i32,
    pub total_files: i32,
    pub total_lines: i32,
    pub total_added: i32,
    pub total_deleted: i32,
    pub total_commits: i32,
    pub first_commit_time: String,
    pub last_commit_time: String,
    pub authors: i32,
}
impl GitBaseInfo {
    pub fn new() -> Self {
        Self {
            project_name: "".to_string(),
            generate_time: "".to_string(),
            age: 0,
            active_days: 0,
            total_files: 0,
            total_lines: 0,
            total_added: 0,
            total_deleted: 0,
            total_commits: 0,
            authors: 0,
            first_commit_time: "".to_string(),
            last_commit_time: "".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentWeeksCommit {
    pub commits_map: HashMap<i32, i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct HoursOfDayCommit {
    pub commits_map: HashMap<i32, i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DayOfWeekCommit {
    pub commits_map: HashMap<i32, i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct MonthOfYearCommit {
    pub commits_map: HashMap<i32, i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct YearAndMonthCommit {
    pub commits_map: HashMap<String, i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct YearCommit {
    pub commits_map: HashMap<i32, i32>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CommmitInfo {
    pub recent_weeks_commit: RecentWeeksCommit,
    pub hours_commit: HoursOfDayCommit,
    pub day_of_week_commit: DayOfWeekCommit,
    pub month_of_year_commit: MonthOfYearCommit,
    pub year_and_month_commit: YearAndMonthCommit,
    pub year_commit: YearCommit,
}
// #[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
// pub struct AuthorStatisticInfoItem {
//     pub author_name: String,
//     pub total_commit: i32,
//     pub total_added: i32,
//     pub total_deleted: i32,
//     pub first_commit: String,
//     pub last_commit: String,
//     pub age: String,
//     pub active_days: i32,
// }
#[derive(Serialize, Deserialize, Clone)]
pub struct TotalAuthorStatisticInfoItem {
    pub author_name: String,
    pub total_commit: i32,
    pub total_added: i32,
    pub total_deleted: i32,
    pub first_commit: String,
    pub last_commit: String,
    pub age: i32,
    pub active_days: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TotalAuthorStatisticInfo {
    pub total_authors: HashMap<String, TotalAuthorStatisticInfoItem>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthorOfMonthStatisticInfo {
    pub authors_map: HashMap<String, HashMap<String, AuthorStatisticInfoItem>>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthorOfYearStatisticInfo {
    pub authors_map: HashMap<String, HashMap<String, AuthorStatisticInfoItem>>,
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AuthorStatisticInfoItem {
    pub author: String,
    pub commit: i32,
}
// Implement Ord and PartialOrd to sort by priority
impl Ord for AuthorStatisticInfoItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.commit.cmp(&other.commit) // Reverse order for max-heap
    }
}

impl PartialOrd for AuthorStatisticInfoItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Clone)]

pub struct AuthorStatisticInfo {
    pub total_author_statistic_info: TotalAuthorStatisticInfo,
    pub author_of_month_statistic_info: AuthorOfMonthStatisticInfo,
    pub author_of_year_statistic_info: AuthorOfYearStatisticInfo,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct FileStatisticInfo {
    pub file_statistic_base_info: FileStatisticBaseInfo,
    pub file_statistic_extension_info: FileStatisticExtensionInfo,
}

#[derive(Serialize, Deserialize, Clone)]

pub struct FileStatisticBaseInfo {
    pub total_files_count: i32,
    pub total_lines_count: i32,
    pub average_file_size: String,
}

#[derive(Serialize, Deserialize, Clone)]

pub struct FileStatisticExtensionInfo {
    pub list: Vec<FileStatisticExtensionInfoItem>,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct FileStatisticExtensionInfoItem {
    pub extention_name: String,
    pub files_count: i32,
    pub lines_count: i32,
}
impl FileStatisticExtensionInfoItem {
    pub fn new(extention_name: String, files_count: i32, lines_count: i32) -> Self {
        Self {
            extention_name,
            files_count,
            lines_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]

pub struct TagStatisticInfo {
    pub tag_statistic_base_info: TagStatisticBaseInfo,
    pub tag_statistic_main_info: TagStatisticMainInfo,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct TagStatisticBaseInfo {
    pub total_tags: i32,
    pub average_commit_per_tag: String,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct TagStatisticMainInfo {
    pub list: Vec<TagStatisticMainInfoItem>,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct TagStatisticMainInfoItem {
    pub tag_name: String,
    pub date: String,
    pub commit_count: i32,
    pub authors: Vec<(String, usize)>,
}
impl TagStatisticMainInfoItem {
    pub fn new(
        tag_name: String,
        date: String,
        commit_count: i32,
        authors: Vec<(String, usize)>,
    ) -> Self {
        Self {
            tag_name,
            date,
            commit_count,
            authors,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]

pub struct LineStatisticInfo {
    pub total_lines: i32,
    pub line_statistic_base_info: HashMap<String, LineStatisticInfoItem>,
    pub directory_loc_info: HashMap<String, HashMap<String, LineStatisticInfoItem>>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]

pub struct LineStatisticInfoItem {
    pub date: String,
    pub count: i32,
}

impl LineStatisticInfoItem {
    fn update(&mut self, total: i32) {
        self.count += total;
    }
    pub fn new(date: String, count: i32) -> Self {
        Self { date, count }
    }
}

#[derive(Serialize, Deserialize, Clone)]

pub struct GitStatisticInfo {
    pub git_base_info: GitBaseInfo,
    pub commit_info: CommmitInfo,
    pub author_statistic_info: AuthorStatisticInfo,
    pub file_statistic_info: FileStatisticInfo,
    pub line_statistic_info: LineStatisticInfo,
    pub tag_statistic_info: TagStatisticInfo,
}
impl GitStatisticInfo {
    pub fn new() -> Self {
        Self {
            git_base_info: GitBaseInfo::new(),
            commit_info: CommmitInfo {
                recent_weeks_commit: RecentWeeksCommit {
                    commits_map: HashMap::new(),
                },
                hours_commit: HoursOfDayCommit {
                    commits_map: HashMap::new(),
                },
                day_of_week_commit: DayOfWeekCommit {
                    commits_map: HashMap::new(),
                },
                month_of_year_commit: MonthOfYearCommit {
                    commits_map: HashMap::new(),
                },
                year_and_month_commit: YearAndMonthCommit {
                    commits_map: HashMap::new(),
                },
                year_commit: YearCommit {
                    commits_map: HashMap::new(),
                },
            },
            author_statistic_info: AuthorStatisticInfo {
                total_author_statistic_info: TotalAuthorStatisticInfo {
                    total_authors: HashMap::new(),
                },
                author_of_month_statistic_info: AuthorOfMonthStatisticInfo {
                    authors_map: HashMap::new(),
                },
                author_of_year_statistic_info: AuthorOfYearStatisticInfo {
                    authors_map: HashMap::new(),
                },
            },
            file_statistic_info: FileStatisticInfo {
                file_statistic_base_info: FileStatisticBaseInfo {
                    total_files_count: 0,
                    total_lines_count: 0,
                    average_file_size: "".to_string(),
                },
                file_statistic_extension_info: FileStatisticExtensionInfo { list: Vec::new() },
            },
            line_statistic_info: LineStatisticInfo {
                total_lines: 0,
                line_statistic_base_info: HashMap::new(),
                directory_loc_info: HashMap::new(),
            },
            tag_statistic_info: TagStatisticInfo {
                tag_statistic_base_info: TagStatisticBaseInfo {
                    total_tags: 0,
                    average_commit_per_tag: "".to_string(),
                },
                tag_statistic_main_info: TagStatisticMainInfo { list: Vec::new() },
            },
        }
    }
    pub fn calc_commit(
        &mut self,
        time: DateTime<Local>,
        author: String,
        total_added: i32,
        total_deleted: i32,
        file_add_del_map: HashMap<String, (i32, i32)>,
        calc_flag: bool,
    ) {
        self.calc_recent_week_commits(time);
        self.calc_hours_commit(time);
        self.calc_day_of_week_commit(time);
        self.calc_month_of_year_commit(time);
        self.calc_year_and_month_commit(time);
        self.calc_year_commit(time);

        self.calc_total_authors(time, author.clone(), total_added, total_deleted);
        self.calc_month_of_year_authors(time, author.clone());
        self.calc_year_authors(time, author.clone());

        self.calc_lines_of_code(
            time,
            total_added,
            total_deleted,
            file_add_del_map,
            calc_flag,
        );
    }
    fn calc_recent_week_commits(&mut self, time: DateTime<Local>) {
        let week = time.iso_week().week() as i32;
        let year = time.iso_week().year();
        let now = Utc::now();
        let now_week = now.iso_week().week() as i32;
        let now_year = now.iso_week().year();

        let week_number = (now_year * 52 + now_week) - (year * 52 + week);

        if !(0..=32).contains(&week_number) {
            return;
        }

        let commit_map = &mut self.commit_info.recent_weeks_commit.commits_map;
        match commit_map.entry(week_number) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = *e.get();
                e.insert(data + 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    fn calc_hours_commit(&mut self, time: DateTime<Local>) {
        let hour = time.hour() as i32;
        let commit_map = &mut self.commit_info.hours_commit.commits_map;
        match commit_map.entry(hour) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = *e.get();
                e.insert(data + 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    fn calc_day_of_week_commit(&mut self, time: DateTime<Local>) {
        let day_of_week = time.date_naive().weekday().number_from_monday() as i32;
        let commit_map = &mut self.commit_info.day_of_week_commit.commits_map;
        match commit_map.entry(day_of_week) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = *e.get();
                e.insert(data + 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    fn calc_month_of_year_commit(&mut self, time: DateTime<Local>) {
        let month = time.month() as i32;
        let commit_map = &mut self.commit_info.month_of_year_commit.commits_map;
        match commit_map.entry(month) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = *e.get();
                e.insert(data + 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    fn calc_year_and_month_commit(&mut self, time: DateTime<Local>) {
        let year_and_month = time.format("%Y-%m").to_string();
        let commit_map = &mut self.commit_info.year_and_month_commit.commits_map;
        match commit_map.entry(year_and_month) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = *e.get();
                e.insert(data + 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    fn calc_year_commit(&mut self, time: DateTime<Local>) {
        let year = time.year();
        let commit_map = &mut self.commit_info.year_commit.commits_map;
        match commit_map.entry(year) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = *e.get();
                e.insert(data + 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    fn calc_total_authors(
        &mut self,
        time: DateTime<Local>,
        author: String,
        total_added: i32,
        total_deleted: i32,
    ) {
        let author_hashmap = &mut self
            .author_statistic_info
            .total_author_statistic_info
            .total_authors;

        let commit_time = time.format("%Y-%m-%d").to_string();
        match author_hashmap.entry(author.clone()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let data = e.get_mut();
                // data.last_commit = commit_time;
                data.total_commit += 1;
                data.total_added += total_added;
                data.total_deleted += total_deleted;
                data.active_days += 1;
                if commit_time < data.first_commit {
                    data.first_commit = commit_time.clone();
                }
                if commit_time > data.last_commit {
                    data.last_commit = commit_time;
                }
                let date1 =
                    NaiveDate::parse_from_str(&data.last_commit, "%Y-%m-%d").unwrap_or_default();
                let date2 =
                    NaiveDate::parse_from_str(&data.first_commit, "%Y-%m-%d").unwrap_or_default();
                data.age = date1.signed_duration_since(date2).num_days() as i32;
                // e.insert(data.clone());
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                // let now = Utc::now();
                // let age = now.signed_duration_since(time);
                let item = TotalAuthorStatisticInfoItem {
                    author_name: author,
                    total_commit: 1,
                    total_added,
                    total_deleted,
                    first_commit: commit_time.clone(),
                    last_commit: commit_time,
                    age: 0,
                    active_days: 1,
                };
                e.insert(item);
            }
        }
    }
    fn calc_month_of_year_authors(&mut self, time: DateTime<Local>, author: String) {
        let author_hashmap = &mut self
            .author_statistic_info
            .author_of_month_statistic_info
            .authors_map;
        let data_str = time.format("%Y-%m").to_string();
        match author_hashmap.entry(data_str.clone()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let map_value = e.get_mut();
                map_value
                    .entry(author.clone())
                    .or_insert(AuthorStatisticInfoItem {
                        author: author.clone(),
                        commit: 0,
                    })
                    .commit += 1;
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                let mut hash_map = HashMap::new();
                hash_map.insert(
                    author.clone(),
                    AuthorStatisticInfoItem { author, commit: 1 },
                );
                e.insert(hash_map);
            }
        }
    }
    fn calc_year_authors(&mut self, time: DateTime<Local>, author: String) {
        let author_hashmap = &mut self
            .author_statistic_info
            .author_of_year_statistic_info
            .authors_map;
        let data_str = time.format("%Y").to_string();
        match author_hashmap.entry(data_str.clone()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let map_value = e.get_mut();
                map_value
                    .entry(author.clone())
                    .or_insert(AuthorStatisticInfoItem {
                        author: author.clone(),
                        commit: 0,
                    })
                    .commit += 1;
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                let mut hash_map = HashMap::new();
                hash_map.insert(
                    author.clone(),
                    AuthorStatisticInfoItem { author, commit: 1 },
                );
                e.insert(hash_map);
            }
        }
    }
    fn calc_lines_of_code(
        &mut self,
        time: DateTime<Local>,
        total_added: i32,
        total_deleted: i32,
        file_add_del_map: HashMap<String, (i32, i32)>,
        calc_flag: bool,
    ) {
        {
            let year_and_month_and_day = time.format("%Y-%m-%d 00:00:00").to_string();
            let commit_map = &mut self.line_statistic_info.directory_loc_info;
            for (file_name, (item_total_added, itemtotal_deleted)) in file_add_del_map {
                let dirs = get_dirs(file_name.as_str());
                for dir in dirs {
                    let total = item_total_added - itemtotal_deleted;
                    let dir_map = commit_map.entry(dir.clone()).or_default();

                    let line_info = dir_map.entry(year_and_month_and_day.clone()).or_insert(
                        LineStatisticInfoItem::new(year_and_month_and_day.clone(), 0),
                    );
                    line_info.update(total);
                }
            }
            // info!("commit_map is {:?}", commit_map);
        }
        if !calc_flag {
            return;
        }
        {
            let total = total_added - total_deleted;
            self.line_statistic_info.total_lines += total;
            let year_and_month_and_day = time.format("%Y-%m-%d 00:00:00").to_string();
            let commit_map = &mut self.line_statistic_info.line_statistic_base_info;
            match commit_map.entry(year_and_month_and_day.clone()) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    let data = e.get_mut();
                    data.count += total;
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(LineStatisticInfoItem {
                        count: total,
                        date: year_and_month_and_day,
                    });
                }
            }
        }
    }
}
impl fmt::Display for GitStatisticInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
pub fn get_dirs(input: &str) -> Vec<String> {
    let mut results = Vec::new();
    let splits = input.split("/").collect::<Vec<&str>>();
    if splits.is_empty() || splits.len() == 1 {
        return results;
    }

    let mut temp = splits[0].to_string();
    results.push(temp.clone());
    for i in 1..(splits.len() - 1) {
        let current = format!("{}/{}", temp, splits[i]);
        results.push(current.clone());
        temp = current;
    }
    // info!("dirs is {:?},src:{}", results, input);
    results
}
