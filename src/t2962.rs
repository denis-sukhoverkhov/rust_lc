// 2962. Count Subarrays Where Max Element Appears at Least K Times
// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut current_num_of_k = 0;

        let max = nums.iter().max().unwrap();

        let mut ans: i64 = 0;
        let mut left: i64 = 0;
        for right in 0..nums.len() {
            if nums[right] == *max {
                current_num_of_k += 1;
            }

            while current_num_of_k == k {
                if nums[left as usize] == *max {
                    current_num_of_k -= 1;
                }
                left += 1;
            }

            ans += left;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays_1() {
        let nums = vec![1, 3, 2, 3, 3];

        assert_eq!(Solution::count_subarrays(nums, 2), 6);
    }

    #[test]
    fn test_count_subarrays_2() {
        let nums = vec![1, 4, 2, 1];

        assert_eq!(Solution::count_subarrays(nums, 3), 0);
    }
}
