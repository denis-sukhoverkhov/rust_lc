// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/description

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut m1 = HashMap::new();
        for c in s.chars() {
            let val = m1.entry(c).or_insert(0);
            *val += 1;
        }

        let mut m2 = HashMap::new();
        for c in t.chars() {
            let val = m2.entry(c).or_insert(0);
            *val += 1;
        }

        if m1.len() != m2.len() {
            return false;
        }

        for (k, v) in m1.iter() {
            if !m2.contains_key(k) {
                return false;
            }

            if m2[k] != *v {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert!(Solution::is_anagram(s, t));
    }

    #[test]
    fn test_is_anagram_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert!(!Solution::is_anagram(s, t));
    }
}
