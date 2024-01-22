// 645. Set Mismatch
// https://leetcode.com/problems/set-mismatch

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let current_summ = nums.iter().sum::<i32>();
        let correct_summ: i32 = ((n * (n + 1)) / 2) as i32;

        let clear_summ: i32 = nums.into_iter().collect::<HashSet<i32>>().iter().sum();

        let missed = correct_summ - clear_summ;
        let duplicated = current_summ - (correct_summ - missed);

        vec![duplicated, missed]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_error_nums_1() {
        let nums = vec![1, 2, 2, 4];
        let expected = vec![2, 3];
        assert_eq!(Solution::find_error_nums(nums), expected);
    }

    #[test]
    fn test_find_error_nums_2() {
        let nums = vec![1, 1];
        let expected = vec![1, 2];
        assert_eq!(Solution::find_error_nums(nums), expected);
    }
}
