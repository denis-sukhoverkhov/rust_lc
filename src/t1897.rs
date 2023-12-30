// 1897. Redistribute Characters to Make All Strings Equal
// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn make_equal(words: Vec<String>) -> bool {
        let ct_words = words.len();
        if ct_words == 1 {
            return true;
        }

        let mut _map = HashMap::new();

        for word in words.iter() {
            for c in word.chars() {
                let val = _map.entry(c).or_insert(0);
                *val += 1;
            }
        }

        for (_, v) in _map {
            if v % ct_words != 0 {
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
    fn test_make_equal_1() {
        let words = vec!["abc".to_string(), "aabc".to_string(), "bc".to_string()];
        assert!(Solution::make_equal(words));
    }

    #[test]
    fn test_make_equal_2() {
        let words = vec!["ab".to_string(), "a".to_string()];
        assert!(!Solution::make_equal(words));
    }

    #[test]
    fn test_make_equal_3() {
        let words = vec!["abbab".to_string()];
        assert!(Solution::make_equal(words));
    }

    #[test]
    fn test_make_equal_4() {
        let words = vec![
            "caaaaa".to_string(),
            "aaaaaaaaa".to_string(),
            "a".to_string(),
            "bbb".to_string(),
            "bbbbbbbbb".to_string(),
            "bbb".to_string(),
            "cc".to_string(),
            "cccccccccccc".to_string(),
            "ccccccc".to_string(),
            "ccccccc".to_string(),
            "cc".to_string(),
            "cccc".to_string(),
            "c".to_string(),
            "cccccccc".to_string(),
            "c".to_string(),
        ];
        assert!(Solution::make_equal(words));
    }
}
