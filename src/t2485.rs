// 2485. Find the Pivot Integer
// https://leetcode.com/problems/find-the-pivot-integer/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn pivot_integer(n: i32) -> i32 {
        let mut prefix_sum = 0;

        let mut prefix_map = HashMap::new();
        for i in 1..=n {
            prefix_sum += i;
            prefix_map.insert(prefix_sum, i);
        }

        let mut postfix_sum = 0;
        for i in (1..=n).rev() {
            postfix_sum += i;

            if prefix_map.contains_key(&postfix_sum) && i == *prefix_map.get(&postfix_sum).unwrap()
            {
                return i;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_integer_1() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }

    #[test]
    fn test_pivot_integer_2() {
        assert_eq!(Solution::pivot_integer(1), 1);
    }

    #[test]
    fn test_pivot_integer_4() {
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
