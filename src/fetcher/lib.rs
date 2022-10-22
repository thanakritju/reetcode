use super::models::Problems;

extern crate reqwest;
extern crate serde_derive;
extern crate serde_json;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";

pub fn get_problems() -> Option<Problems> {
    let res = reqwest::blocking::get(PROBLEMS_URL).unwrap();
    let problems = res.json::<Problems>().unwrap();
    return Some(problems);
}
