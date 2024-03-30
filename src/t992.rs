// 992. Subarrays with K Different Integers
// https://leetcode.com/problems/subarrays-with-k-different-integers/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut m = HashMap::new();

        let (mut l_far, mut l_near) = (0, 0);
        for rpt in 0..nums.len() {
            *m.entry(nums[rpt]).or_insert(0) += 1;

            while m.len() > k as usize {
                let v = m.get_mut(&nums[l_near]).unwrap();
                *v -= 1;

                if *v == 0 {
                    m.remove(&nums[l_near]);
                    l_near += 1;
                }
                l_far = l_near;
            }

            loop {
                let v = m.get_mut(&nums[l_near]).unwrap();

                if *v <= 1 {
                    break;
                }

                *v -= 1;
                l_near += 1;
            }

            if m.len() == k as usize {
                res += l_near - l_far + 1;
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarrays_with_k_distinct_1() {
        let nums = vec![1, 2, 1, 2, 3];

        assert_eq!(Solution::subarrays_with_k_distinct(nums, 2), 7);
    }

    #[test]
    fn test_subarrays_with_k_distinct_2() {
        let nums = vec![1, 2, 1, 3, 4];

        assert_eq!(Solution::subarrays_with_k_distinct(nums, 3), 3);
    }
}
