pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest = String::from("");
        let first = &strs[0];
        for (i, c) in first.chars().enumerate() {
            for s in &strs[1..] {
                if let Some(my_char) = s.chars().nth(i) {
                    if c != my_char {
                        return longest;
                    }
                } else {
                    return longest;
                }
            }
            longest.push(c);
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
    }
}
