// 300. Longest Increasing Subsequence
// https://leetcode.com/problems/longest-increasing-subsequence/description

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lis: Vec<i32> = vec![1; nums.len()];

        let mut resp = 1;
        for i in (0..nums.len()).rev() {
            for j in (i + 1)..nums.len() {
                if nums[j] > nums[i] {
                    lis[i] = std::cmp::max(lis[i], 1 + lis[j]);
                }
            }

            resp = std::cmp::max(resp, lis[i]);
        }

        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis_1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];

        assert_eq!(Solution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_length_of_lis_2() {
        let nums = vec![0, 1, 0, 3, 2, 3];

        assert_eq!(Solution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_length_of_lis_3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];

        assert_eq!(Solution::length_of_lis(nums), 1);
    }
}
