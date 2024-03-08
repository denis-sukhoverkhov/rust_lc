// 3005. Count Elements With Maximum Frequency
// https://leetcode.com/problems/count-elements-with-maximum-frequency/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();

        let mut max_occurrences = 0;
        for i in nums.iter() {
            let v = m.entry(i).or_insert(0);
            *v += 1;

            max_occurrences = max_occurrences.max(*v);
        }

        let mut res = 0;
        for (_, v) in m.iter() {
            if *v == max_occurrences {
                res += *v;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_frequency_elements_1() {
        let nums = vec![1, 2, 2, 3, 1, 4];

        assert_eq!(Solution::max_frequency_elements(nums), 4);
    }

    #[test]
    fn test_max_frequency_elements_2() {
        let nums = vec![1, 2, 3, 4, 5];

        assert_eq!(Solution::max_frequency_elements(nums), 5);
    }
}
