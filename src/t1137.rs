// 1137. N-th Tribonacci Number
// https://leetcode.com/problems/n-th-tribonacci-number/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn tribonacci(n: i32) -> i32 {
        let mut res = [0, 1, 1];
        for _ in 0..n {
            let tmp = res.iter().sum();
            res[0] = res[1];
            res[1] = res[2];
            res[2] = tmp;
        }

        res[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci_1() {
        assert_eq!(Solution::tribonacci(4), 4)
    }

    #[test]
    fn test_tribonacci_2() {
        assert_eq!(Solution::tribonacci(25), 1389537)
    }
}
