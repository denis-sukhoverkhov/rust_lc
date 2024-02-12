// 169. Majority Element
// https://leetcode.com/problems/majority-element

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();

        let mut max = 0;
        let mut val = 0;
        for n in nums.iter() {
            let v = m.entry(n).or_insert(0);
            *v += 1;

            if *v > max {
                max = *v;
                val = *n;
            }
        }

        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element_1() {
        let nums = vec![3, 2, 3];

        assert_eq!(Solution::majority_element(nums), 3);
    }

    #[test]
    fn test_majority_element_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];

        assert_eq!(Solution::majority_element(nums), 2);
    }
}
