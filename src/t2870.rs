// 2870. Minimum Number of Operations to Make Array Empty
// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();

        for n in nums {
            let ct = m.entry(n).or_insert(0);
            *ct += 1;
        }

        let mut res = 0;
        for (_, v) in m.iter() {
            if *v == 1 {
                return -1;
            } else if *v == 2 {
                res += 1;
                continue;
            } else if *v == 4 {
                res += 2;
                continue;
            }

            let remainder = *v % 3;
            let division = v / 3;
            if remainder != 0 {
                res += 1;
            }

            res += division;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_1() {
        let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];

        assert_eq!(Solution::min_operations(nums), 4)
    }

    #[test]
    fn test_min_operations_2() {
        let nums = vec![2, 1, 2, 2, 3, 3];

        assert_eq!(Solution::min_operations(nums), -1)
    }
}
