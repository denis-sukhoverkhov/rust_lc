// 2958. Length of Longest Subarray With at Most K Frequency
// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (0i32, 0i32);

        let mut resp = 0;
        let mut m = HashMap::new();

        while r < nums.len() as i32 {
            let val = m.entry(nums[r as usize]).or_insert(0);
            *val += 1;

            let mut val = *val;

            while val > k {
                let el = m.get_mut(&nums[l as usize]).unwrap();
                *el -= 1;

                if nums[l as usize] == nums[r as usize] {
                    val -= 1;
                }

                l += 1;
            }

            resp = resp.max(r - l + 1);

            r += 1;
        }

        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_length_1() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];

        assert_eq!(Solution::max_subarray_length(nums, 2), 6)
    }

    #[test]
    fn test_max_subarray_length_2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];

        assert_eq!(Solution::max_subarray_length(nums, 1), 2)
    }

    #[test]
    fn test_max_subarray_length_3() {
        let nums = vec![5, 5, 5, 5, 5, 5, 5];

        assert_eq!(Solution::max_subarray_length(nums, 4), 4)
    }
}
