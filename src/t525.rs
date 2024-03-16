// 525. Contiguous Array
// https://leetcode.com/problems/contiguous-array/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut m = HashMap::new();
        let mut res: i32 = 0;

        for (i, v) in nums.iter().enumerate() {
            if *v == 0 {
                diff -= 1;
            } else {
                diff += 1;
            }

            if !m.contains_key(&diff) {
                let v = m.entry(diff).or_insert(0);
                *v = i as i32;
            }

            if diff == 0 {
                res = i as i32 + 1;
            } else {
                let idx = m.get(&diff).unwrap();
                res = res.max((i as i32) - *idx);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_max_length_1() {
        let nums = vec![0, 1];
        assert_eq!(Solution::find_max_length(nums), 2);
    }

    #[test]
    fn test_find_max_length_2() {
        let nums = vec![0, 1, 0];
        assert_eq!(Solution::find_max_length(nums), 2);
    }

    #[test]
    fn test_find_max_length_3() {
        let nums = vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1];
        assert_eq!(Solution::find_max_length(nums), 14);
    }

    #[test]
    fn test_find_max_length_4() {
        let nums = vec![1, 1, 1, 1, 1, 0, 0, 0, 0];
        assert_eq!(Solution::find_max_length(nums), 8);
    }

    #[test]
    fn test_find_max_length_5() {
        let nums = vec![1, 1, 1, 1, 1, 0, 0];
        assert_eq!(Solution::find_max_length(nums), 4);
    }

    #[test]
    fn test_find_max_length_6() {
        let nums = vec![0, 1, 1, 0, 1, 1, 1, 0];
        assert_eq!(Solution::find_max_length(nums), 4);
    }

    #[test]
    fn test_find_max_length_7() {
        let nums = vec![0, 0];
        assert_eq!(Solution::find_max_length(nums), 0);
    }

    #[test]
    fn test_find_max_length_8() {
        let nums = vec![
            0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1,
            0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1,
            0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1,
        ];
        let t: i32 = nums.iter().sum();
        dbg!(t);
        dbg!(nums.len());
        assert_eq!(Solution::find_max_length(nums), 68);
    }
}
