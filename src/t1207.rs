// 1207. Unique Number of Occurrences
// https://leetcode.com/problems/unique-number-of-occurrences

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut m = HashMap::new();

        for v in arr.iter() {
            let o = m.entry(v).or_insert(0);
            *o += 1;
        }

        let mut s = HashSet::new();
        for (_, v) in m.iter() {
            s.insert(v);
        }

        s.len() == m.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_occurrences_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];

        assert!(Solution::unique_occurrences(arr))
    }

    #[test]
    fn test_unique_occurrences_2() {
        let arr = vec![1, 2];

        assert!(!Solution::unique_occurrences(arr))
    }

    #[test]
    fn test_unique_occurrences_3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];

        assert!(Solution::unique_occurrences(arr))
    }
}
