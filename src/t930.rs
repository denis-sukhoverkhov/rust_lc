// 930. Binary Subarrays With Sum
// https://leetcode.com/problems/binary-subarrays-with-sum/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut res = 0;

        let mut prefix_sum = 0;
        let mut m = HashMap::new();

        (0..nums.len()).for_each(|i| {
            prefix_sum += nums[i];

            if prefix_sum == goal {
                res += 1;
            }

            let key = prefix_sum - goal;
            if m.contains_key(&key) {
                res += m.get(&key).unwrap();
            }

            let val = m.entry(prefix_sum).or_insert(0);
            *val += 1;
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarrays_with_sum_1() {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), 4);
    }

    #[test]
    fn test_num_subarrays_with_sum_2() {
        let nums = vec![0, 0, 0, 0, 0];
        let goal = 0;
        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), 15);
    }
}
