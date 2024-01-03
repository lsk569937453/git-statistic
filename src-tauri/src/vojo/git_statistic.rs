use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::Timelike;
use chrono::Utc;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use tauri::State;
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

pub struct GitStatisticInfo {
    pub git_base_info: GitBaseInfo,
    pub recent_weeks_commit: RecentWeeksCommit,
    pub hours_commit: HoursOfDayCommit,
    pub day_of_week_commit: DayOfWeekCommit,
    pub month_of_year_commit: MonthOfYearCommit,
    pub year_and_month_commit: YearAndMonthCommit,
    pub year_commit: YearCommit,
}
impl GitStatisticInfo {
    pub fn new() -> Self {
        Self {
            git_base_info: GitBaseInfo::new(),
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
        }
    }
    pub fn calc(&mut self, time: DateTime<Local>) {
        self.calc_recent_week_commits(time);
        self.calc_hours_commit(time);
        self.calc_day_of_week_commit(time);
        self.calc_month_of_year_commit(time);
        self.calc_year_and_month_commit(time);
        self.calc_year_commit(time);
    }
    fn calc_recent_week_commits(&mut self, time: DateTime<Local>) {
        let now = Local::now();
        let duration = now - time;
        let week = duration.num_weeks() as i32;
        if week < 0 || week > 32 {
            return;
        }

        let commit_map = &mut self.recent_weeks_commit.commits_map;
        match commit_map.entry(week) {
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
        let commit_map = &mut self.hours_commit.commits_map;
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
        let commit_map = &mut self.day_of_week_commit.commits_map;
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
        let commit_map = &mut self.month_of_year_commit.commits_map;
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
        let commit_map = &mut self.year_and_month_commit.commits_map;
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
        let commit_map = &mut self.year_commit.commits_map;
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
}
impl fmt::Debug for GitStatisticInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("GitStatisticInfo");
        s.field("recent_weeks_commit", &self.recent_weeks_commit);
        s.field("hours_commit", &self.hours_commit);
        s.field("day_of_week_commit", &self.day_of_week_commit);
        s.field("month_of_year_commit", &self.month_of_year_commit);
        s.field("year_and_month_commit", &self.year_and_month_commit);
        s.field("year_commit", &self.year_commit);
        s.finish()
    }
}
impl fmt::Display for GitStatisticInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("GitStatisticInfo");
        s.field("recent_weeks_commit", &self.recent_weeks_commit);
        s.field("hours_commit", &self.hours_commit);
        s.field("day_of_week_commit", &self.day_of_week_commit);
        s.field("month_of_year_commit", &self.month_of_year_commit);
        s.field("year_and_month_commit", &self.year_and_month_commit);
        s.field("year_commit", &self.year_commit);
        s.finish()
    }
}
