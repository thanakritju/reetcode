use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::fetcher::lib::get_problems;
use crate::fetcher::lib::get_slug_name_by_id;

pub fn generate_template(id: i64) -> bool {
    println!("Generating leetcode template for id {id}");
    let problems = get_problems().unwrap();
    let slugname = get_slug_name_by_id(id, problems.stat_status_pairs).unwrap();
    if slugname == "" {
        eprintln!("Invalid id: {id}");
        return false;
    }
    let filename = format!("s{:04}_{}.rs", id, slugname.replace("-", "_"));
    println!("Filename: {filename}");
    update_mod_file(filename.replace(".rs", ""));
    create_solution_file(filename);
    true
}

fn update_mod_file(content: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/solution/mod.rs")
        .unwrap();

    if let Err(e) = writeln!(file, "mod {};", content) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn create_solution_file(filename: String) {
    let mut file = File::create(format!("src/solution/{}", filename))
        .expect("Error encountered while creating file!");
    file.write_all(
        b"pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}",
    )
    .expect("Error while writing to file");
}
