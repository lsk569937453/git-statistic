use std::collections::BinaryHeap;
use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::git_statistic::AuthorStatisticInfoItem;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorOfMonthResponse {
    pub data: Vec<AuthorOfMonthResponseItem>,
}
impl AuthorOfMonthResponse {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn from_hashmap(
        hash_map: HashMap<String, HashMap<String, AuthorStatisticInfoItem>>,
    ) -> Self {
        let mut data = vec![];
        for (key, value) in hash_map {
            let date = key;
            let mut total_commit_count = 0;
            let count_of_author = value.len() as i32;
            let mut heap = BinaryHeap::new();
            for (_, item) in value {
                let current_count = item.commit;
                total_commit_count += current_count;
                heap.push(item);
            }
            let (auther_name, count_of_commit_of_author) = if let Some(item) = heap.pop() {
                (item.author, item.commit)
            } else {
                ("".to_string(), 0)
            };
            let mut next_top_five = vec![];
            while let Some(task) = heap.pop() {
                next_top_five.push(task.author.clone());
            }
            data.push(AuthorOfMonthResponseItem::new(
                date,
                auther_name,
                count_of_commit_of_author,
                total_commit_count,
                count_of_author,
                next_top_five,
            ))
        }
        data.sort_by(|a, b| b.date.cmp(&a.date));

        Self { data }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorOfMonthResponseItem {
    pub date: String,
    pub author_name: String,
    pub count_of_commit_of_author: i32,
    pub total_commit_count: i32,
    pub count_of_author: i32,
    pub next_top_five: Vec<String>,
}
impl AuthorOfMonthResponseItem {
    pub fn new(
        date: String,
        author_name: String,
        count_of_commit_of_author: i32,
        total_commit_count: i32,
        count_of_author: i32,
        next_top_five: Vec<String>,
    ) -> Self {
        Self {
            date,
            author_name,
            count_of_commit_of_author,
            total_commit_count,
            count_of_author,
            next_top_five,
        }
    }
}
