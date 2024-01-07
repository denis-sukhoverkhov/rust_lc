// 446. Arithmetic Slices II - Subsequence
// https://leetcode.com/problems/arithmetic-slices-ii-subsequence

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); nums_len];

        let mut res = 0;
        for (i, _) in nums.iter().enumerate() {
            for j in 0..i {
                let diff: i64 = nums[i] as i64 - nums[j] as i64;
                if let Some(hm) = dp.get(j) {
                    let mut prev_value = 0;
                    if let Some(v) = hm.get(&diff) {
                        prev_value = *v;
                    }

                    let val = dp[i].entry(diff).or_insert(0);
                    *val += 1 + prev_value;

                    res += prev_value;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_arithmetic_slices_1() {
        let nums = vec![2, 4, 6, 8, 10];

        assert_eq!(Solution::number_of_arithmetic_slices(nums), 7);
    }

    #[test]
    fn test_number_of_arithmetic_slices_2() {
        let nums = vec![7, 7, 7, 7, 7];

        assert_eq!(Solution::number_of_arithmetic_slices(nums), 16);
    }

    #[test]
    fn test_number_of_arithmetic_slices_3() {
        let nums = vec![0, 2000000000, -294967296];

        assert_eq!(Solution::number_of_arithmetic_slices(nums), 0);
    }
}
