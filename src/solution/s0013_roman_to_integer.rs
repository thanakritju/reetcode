pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut prev_char = 'A';
        let mut ans = 0;
        for char in s.chars() {
            match char {
                'I' => ans += 1,
                'V' => {
                    ans += match prev_char {
                        'I' => 3,
                        _ => 5,
                    }
                }
                'X' => {
                    ans += match prev_char {
                        'I' => 8,
                        _ => 10,
                    }
                }
                'L' => {
                    ans += match prev_char {
                        'X' => 30,
                        _ => 50,
                    }
                }
                'C' => {
                    ans += match prev_char {
                        'X' => 80,
                        _ => 100,
                    }
                }
                'D' => {
                    ans += match prev_char {
                        'C' => 300,
                        _ => 500,
                    }
                }
                'M' => {
                    ans += match prev_char {
                        'C' => 800,
                        _ => 1000,
                    }
                }
                _ => eprintln!("Invalid format"),
            }
            prev_char = char;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::roman_to_int("I".to_string()));
        assert_eq!(2, Solution::roman_to_int("II".to_string()));
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(4, Solution::roman_to_int("IV".to_string()));
        assert_eq!(5, Solution::roman_to_int("V".to_string()));
        assert_eq!(6, Solution::roman_to_int("VI".to_string()));
        assert_eq!(9, Solution::roman_to_int("IX".to_string()));
        assert_eq!(10, Solution::roman_to_int("X".to_string()));
        assert_eq!(11, Solution::roman_to_int("XI".to_string()));
        assert_eq!(27, Solution::roman_to_int("XXVII".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(40, Solution::roman_to_int("XL".to_string()));
        assert_eq!(60, Solution::roman_to_int("LX".to_string()));
        assert_eq!(900, Solution::roman_to_int("CM".to_string()));
        assert_eq!(400, Solution::roman_to_int("CD".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
