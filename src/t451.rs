// 451. Sort Characters By Frequency
// https://leetcode.com/problems/sort-characters-by-frequency

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn frequency_sort(s: String) -> String {
        let mut m = HashMap::new();

        for c in s.chars() {
            let v = m.entry(c).or_insert(0);
            *v += 1;
        }

        let mut parts = vec![];

        for (k, v) in m.iter() {
            parts.push(k.to_string().repeat(*v as usize));
        }

        parts.sort();
        parts.sort_by_key(|b| std::cmp::Reverse(b.chars().count()));

        parts.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort_1() {
        let s = "tree".to_string();
        let expected = "eert".to_string();

        assert_eq!(Solution::frequency_sort(s), expected);
    }

    #[test]
    fn test_frequency_sort_2() {
        let s = "cccaaa".to_string();
        let expected = "aaaccc".to_string();

        assert_eq!(Solution::frequency_sort(s), expected);
    }

    #[test]
    fn test_frequency_sort_3() {
        let s = "Aabb".to_string();
        let expected = "bbAa".to_string();

        assert_eq!(Solution::frequency_sort(s), expected);
    }
}
