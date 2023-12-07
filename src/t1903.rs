// 1903. Largest Odd Number in String
// https://leetcode.com/problems/largest-odd-number-in-string/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn largest_odd_number(num: String) -> String {
        for (i, c) in num.chars().rev().enumerate() {
            if c.to_digit(10).unwrap() % 2 == 1 {
                return num[..num.len() - i].to_string();
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_odd_number_1() {
        assert_eq!(
            Solution::largest_odd_number("52".to_string()),
            "5".to_string()
        );
    }

    #[test]
    fn test_largest_odd_number_2() {
        assert_eq!(
            Solution::largest_odd_number("4206".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn test_largest_odd_number_3() {
        assert_eq!(
            Solution::largest_odd_number("35427".to_string()),
            "35427".to_string()
        );
    }
}
