// 49. Group Anagrams
// https://leetcode.com/problems/group-anagrams

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut key: [u32; 26] = [0; 26];

            for c in s.chars() {
                let idx = (c as u32) - 'a' as u32;
                key[idx as usize] += 1;
            }

            let v = m.entry(key).or_default();
            v.push(s.clone());
        }

        m.into_iter().fold(vec![], |mut v, (_, val)| {
            v.push(val);

            v
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams_1() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let excpected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        let mut actual = Solution::group_anagrams(strs);

        actual.sort_by_key(|a| a.len());

        for a in actual.iter_mut() {
            a.sort()
        }

        assert_eq!(actual, excpected);
    }

    #[test]
    fn test_group_anagrams_2() {
        let strs = vec!["".to_string()];

        let excpected = vec![vec!["".to_string()]];

        assert_eq!(Solution::group_anagrams(strs), excpected);
    }

    #[test]
    fn test_group_anagrams_3() {
        let strs = vec!["a".to_string()];

        let excpected = vec![vec!["a".to_string()]];

        assert_eq!(Solution::group_anagrams(strs), excpected);
    }
}
