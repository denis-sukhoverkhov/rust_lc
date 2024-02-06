// 387. First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut h = HashMap::new();

        for c in s.chars() {
            let v = h.entry(c).or_insert(0);
            *v += 1;
        }

        for (i, c) in s.chars().enumerate() {
            let v = h.get(&c).unwrap();

            if *v == 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_uniq_char_1() {
        let s = String::from("leetcode");

        assert_eq!(Solution::first_uniq_char(s), 0);
    }

    #[test]
    fn test_first_uniq_char_2() {
        let s = String::from("loveleetcode");

        assert_eq!(Solution::first_uniq_char(s), 2);
    }

    #[test]
    fn test_first_uniq_char_3() {
        let s = String::from("aabb");

        assert_eq!(Solution::first_uniq_char(s), -1);
    }
}
