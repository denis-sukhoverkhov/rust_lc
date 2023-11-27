// 935. Knight Dialer
// https://leetcode.com/problems/knight-dialer/description/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn knight_dialer(n: i32) -> i32 {
        if n == 1 {
            return 10;
        }

        let mut map = HashMap::new();
        map.insert(0, vec![4, 6]);
        map.insert(1, vec![6, 8]);
        map.insert(2, vec![7, 9]);
        map.insert(3, vec![4, 8]);
        map.insert(4, vec![0, 3, 9]);
        map.insert(5, vec![]);
        map.insert(6, vec![0, 1, 7]);
        map.insert(7, vec![2, 6]);
        map.insert(8, vec![1, 3]);
        map.insert(9, vec![2, 4]);

        let mut dp = vec![1; 10];
        for _ in 1..n {
            let mut next_dp: Vec<i32> = vec![0; 10];
            for (i, v) in next_dp.iter_mut().enumerate() {
                for next in map.get(&i).unwrap() {
                    *v = (*v + dp[*next]) % 1_000_000_007;
                }
            }
            dp = next_dp;
        }
        let mut summ: i32 = 0;
        for v in dp.iter() {
            summ = (summ + *v) % 1_000_000_007;
        }

        summ % 1_000_000_007
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_dialer_1() {
        assert_eq!(Solution::knight_dialer(1), 10);
    }

    #[test]
    fn test_knight_dialer_2() {
        assert_eq!(Solution::knight_dialer(2), 20);
    }

    #[test]
    fn test_knight_dialer_3131() {
        assert_eq!(Solution::knight_dialer(3131), 136006598);
    }
}
