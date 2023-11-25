// 1685. Sum of Absolute Differences in a Sorted Array
// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/

#[derive(Default)]
struct Solution;

impl Solution {

    #[allow(dead_code)]
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        let mut prefix_sum = vec![nums[0]; nums.len()];
        for i in 1..nums.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
        }

        let mut result = vec![0; nums.len()];

        // nums = [1,4,6,8,10]
        // prefix_sum = [1,5,11,19,29]
        // i = 1
        // 4 - 1 + 4 - 4 + 6 - 4 + 8 - 4 + 10 - 4
        // (i + 1) * 4 - (1 + 4) - first part
        // (6 + 8 + 10) - (n - (i+1) * 4) - second part
        // (i + 1) * nums[i] - prefix_sum[i] + (prefix_sum[n-1] - prefix_sum[i]) - (n - (i+1)) * nums[i]

        for i in 0..nums.len() {
            let len_first_part = (i + 1) as i32;

            let first_part = len_first_part * nums[i] - prefix_sum[i];
            let postfix_sum = prefix_sum[nums.len() - 1] - prefix_sum[i];

            let len_second_part = nums.len() as i32 - len_first_part;
            let second_part = postfix_sum - (len_second_part * nums[i]);

            result[i] = first_part + second_part;
        }

        result
    }
}

// write unit tests to test above fn
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_absolute_differences() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5]),
            vec![4, 3, 5]
        );
    }

    #[test]
    fn test_get_sum_absolute_differences2() {
        let nums = vec![1, 4, 6, 8, 10];

        let should_be = vec![24, 15, 13, 15, 21];
        assert_eq!(Solution::get_sum_absolute_differences(nums), should_be);
    }
}
