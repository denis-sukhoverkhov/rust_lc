// 198. House Robber
// https://leetcode.com/problems/house-robber

#[derive(Default)]
struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();

        let (mut r1, mut r2) = (0, 0);
        for i in nums.iter() {
            let tmp = (i + r1).max(r2);
            r1 = r2;
            r2 = tmp;
        }

        r2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob_1() {
        let nums = vec![1, 2, 3, 1];

        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn test_rob_2() {
        let nums = vec![2, 7, 9, 3, 1];

        assert_eq!(Solution::rob(nums), 12);
    }

    #[test]
    fn test_rob_3() {
        let nums = vec![2, 1, 1, 2];

        assert_eq!(Solution::rob(nums), 4);
    }
}
