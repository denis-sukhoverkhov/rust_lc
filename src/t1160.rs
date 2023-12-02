// 1160. Find Words That Can Be Formed by Characters
// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_map = HashMap::new();
        for c in chars.chars() {
            let count = chars_map.entry(c).or_insert(0);
            *count += 1;
        }

        let mut answer = 0;

        for word in words.iter() {
            let mut word_map = HashMap::new();
            for w in word.chars() {
                let ct = word_map.entry(w).or_insert(0);
                *ct += 1;
            }

            let mut is_good = true;
            for (k, v) in word_map.iter() {
                if !chars_map.contains_key(k) || chars_map[k] < *v {
                    is_good = false;
                    break;
                }
            }

            if !is_good {
                continue;
            }
            answer += word.chars().count() as i32;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_strings_are_equal_1() {
        let words = vec![
            "cat".to_string(),
            "bt".to_string(),
            "hat".to_string(),
            "tree".to_string(),
        ];
        let chars = "atach".to_string();

        assert_eq!(Solution::count_characters(words, chars), 6)
    }

    #[test]
    fn test_array_strings_are_equal_2() {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "leetcode".to_string(),
        ];
        let chars = "welldonehoneyr".to_string();

        assert_eq!(Solution::count_characters(words, chars), 10)
    }
}
