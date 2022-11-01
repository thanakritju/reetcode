use super::models::*;

extern crate reqwest;
extern crate serde_derive;
extern crate serde_json;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";

pub fn get_problems() -> Option<Problems> {
    let res = reqwest::blocking::get(PROBLEMS_URL).unwrap();
    let problems = res.json::<Problems>().unwrap();
    return Some(problems);
}

pub fn get_slug_name_by_id(id: i64, problems: Vec<StatStatusPair>) -> Option<String> {
    for problem in problems {
        if problem.stat.question_id == id {
            return problem.stat.question_article_slug;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::fetcher::{lib::get_slug_name_by_id, models::Problems};

    #[test]
    fn test_get_slug_name_by_id() {
        let data = fs::read_to_string("./src/fetcher/test_data.json").expect("Unable to read file");
        let problems: Problems =
            serde_json::from_str(&data).expect("JSON does not have correct format.");

        let out = get_slug_name_by_id(1, problems.stat_status_pairs).unwrap();

        assert_eq!(out, "two-sum");
    }
}
