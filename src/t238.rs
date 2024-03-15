// 238. Product of Array Except Self
// https://leetcode.com/problems/product-of-array-except-self/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut prefix = 1;
        for i in 0..nums.len() {
            res[i] = prefix;
            prefix *= nums[i];
        }

        let mut postfix = 1;
        for i in (0..nums.len()).rev() {
            res[i] *= postfix;
            postfix *= nums[i];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self_1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];

        assert_eq!(Solution::product_except_self(nums), expected);
    }

    #[test]
    fn test_product_except_self_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];

        assert_eq!(Solution::product_except_self(nums), expected);
    }
}
