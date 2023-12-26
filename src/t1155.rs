// 1155. Number of Dice Rolls With Target Sum
// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut cache = HashMap::new();

        fn ct(n: i32, t: i32, k: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
            let module = 1_000_000_000 + 7;

            if cache.contains_key(&(n, t)) {
                return *cache.get(&(n, t)).unwrap();
            }

            if n == 0 && t != 0 {
                return 0;
            }

            if n == 0 && t == 0 {
                return 1;
            }

            let mut res = 0;

            for i in 1..k + 1 {
                res = (res + ct(n - 1, t - i, k, cache)) % module;
            }
            cache.insert((n, t), res);

            res
        }

        ct(n, target, k, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rolls_to_target_1() {
        let n = 1;
        let k = 6;
        let target = 3;

        assert_eq!(Solution::num_rolls_to_target(n, k, target), 1)
    }

    #[test]
    fn test_num_rolls_to_target_2() {
        let n = 2;
        let k = 6;
        let target = 7;

        assert_eq!(Solution::num_rolls_to_target(n, k, target), 6)
    }

    #[test]
    fn test_num_rolls_to_target_3() {
        let n = 30;
        let k = 30;
        let target = 500;

        assert_eq!(Solution::num_rolls_to_target(n, k, target), 222616187)
    }
}
