// 1347. Minimum Number of Steps to Make Two Strings Anagram
// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_steps(s: String, t: String) -> i32 {
        let smap = Solution::str_to_hashmap(&s);
        let tmap = Solution::str_to_hashmap(&t);

        let mut res = 0;
        for (k, v) in smap.iter() {
            let tval = tmap.get(k).unwrap_or(&0);

            let diff = *v - *tval;
            if diff > 0 {
                res += diff;
            }
        }

        res
    }

    fn str_to_hashmap(s: &str) -> HashMap<char, i32> {
        let mut smap: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            let ct = smap.entry(c).or_insert(0);
            *ct += 1;
        }

        smap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_steps_1() {
        let (s, t) = ("bab".to_string(), "aba".to_string());

        assert_eq!(Solution::min_steps(s, t), 1);
    }

    #[test]
    fn test_min_steps_2() {
        let (s, t) = ("leetcode".to_string(), "practice".to_string());

        assert_eq!(Solution::min_steps(s, t), 5);
    }

    #[test]
    fn test_min_steps_3() {
        let (s, t) = ("anagram".to_string(), "mangaar".to_string());

        assert_eq!(Solution::min_steps(s, t), 0);
    }
}
