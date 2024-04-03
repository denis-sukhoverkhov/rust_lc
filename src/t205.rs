// 205. Isomorphic Strings
// https://leetcode.com/problems/isomorphic-strings/

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let mut s_map = HashMap::new();

        for i in 0..s.len() {
            let ch1 = s[i];
            let ch2 = t[i];
            let val = s_map.entry(ch1).or_insert(ch2);

            if *val != ch2 {
                return false;
            }
        }

        s_map.keys().count() == s_map.values().collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic_1() {
        let s = String::from("egg");
        let t = String::from("add");

        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn test_is_isomorphic_2() {
        let s = String::from("foo");
        let t = String::from("bar");

        assert!(!Solution::is_isomorphic(s, t));
    }

    #[test]
    fn test_is_isomorphic_3() {
        let s = String::from("paper");
        let t = String::from("title");

        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn test_is_isomorphic_4() {
        let s = String::from("bbbaaaba");
        let t = String::from("aaabbbba");

        assert!(!Solution::is_isomorphic(s, t));
    }

    #[test]
    fn test_is_isomorphic_5() {
        let s = String::from("badc");
        let t = String::from("baba");

        assert!(!Solution::is_isomorphic(s, t));
    }
}
