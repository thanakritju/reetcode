pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else {
            let s = x.to_string();
            let mut reverse = String::from("");
            for c in s.chars() {
                reverse = format!("{}{}", c, reverse);
            }
            reverse == s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, Solution::is_palindrome(121,));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
