// 1758. Minimum Changes To Make Alternating Binary String
// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_operations(s: String) -> i32 {
        let opposite_map = HashMap::from([('0', '1'), ('1', '0')]);

        let mut result = s.chars().count() as i32;
        for ch in "10".chars() {
            let mut prev = ch;

            let mut op = 0;
            for c in s.chars() {
                if prev == c {
                    op += 1;
                    prev = opposite_map[&prev];
                } else {
                    prev = c;
                }
            }

            result = result.min(op);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_1() {
        let s = String::from("0100");

        assert_eq!(Solution::min_operations(s), 1);
    }

    #[test]
    fn test_min_operations_2() {
        let s = String::from("10");

        assert_eq!(Solution::min_operations(s), 0);
    }

    #[test]
    fn test_min_operations_3() {
        let s = String::from("1111");

        assert_eq!(Solution::min_operations(s), 2);
    }
}
