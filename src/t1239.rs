// 1239. Maximum Length of a Concatenated String with Unique Characters
// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_length(arr: Vec<String>) -> i32 {
        fn has_duplicates(h1: &HashSet<char>, t: &str) -> bool {
            let h2: HashSet<char> = t.chars().collect();
            if h2.len() != t.len() {
                return true;
            }

            let it: HashSet<_> = h1.intersection(&h2).collect();

            if !it.is_empty() {
                return true;
            }

            false
        }

        fn backtrace(i: usize, arr: &[String], s: &mut HashSet<char>) -> i32 {
            if i == arr.len() {
                return s.len() as i32;
            }

            let mut res: i32 = 0;
            if !has_duplicates(s, &arr[i]) {
                for c in arr[i].chars() {
                    s.insert(c);
                }

                res = backtrace(i + 1, arr, s);

                for c in arr[i].chars() {
                    s.remove(&c);
                }
            }

            std::cmp::max(res, backtrace(i + 1, arr, s))
        }

        let mut s = HashSet::new();
        backtrace(0, &arr, &mut s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_length_1() {
        let arr = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];

        assert_eq!(Solution::max_length(arr), 4)
    }

    #[test]
    fn test_max_length_2() {
        let arr = vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string(),
        ];

        assert_eq!(Solution::max_length(arr), 6)
    }

    #[test]
    fn test_max_length_3() {
        let arr = vec!["abcdefghijklmnopqrstuvwxyz".to_string()];

        assert_eq!(Solution::max_length(arr), 26)
    }
}
