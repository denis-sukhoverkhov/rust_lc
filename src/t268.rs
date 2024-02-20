// 268. Missing Number
// https://leetcode.com/problems/missing-number/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut s = 0_i32;

        for i in 0..=nums.len() {
            s += i as i32;
        }

        let curr_s: i32 = nums.iter().sum();

        s - curr_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number_1() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn test_missing_number_2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn test_missing_number_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
    }
}
