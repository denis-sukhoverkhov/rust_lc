// 191. Number of 1 Bits
// https://leetcode.com/problems/number-of-1-bits/description/?envType=daily-question&envId=2023-11-29

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code, non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let mut result = 0;

        let mut curr = n;
        while curr != 0 {
            result += 1;
            curr &= curr - 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::hammingWeight(11), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::hammingWeight(128), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::hammingWeight(4294967293), 31);
    }
}
