// 1464. Maximum Product of Two Elements in an Array
// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2) = (-1, -1);
        // let (mut idx_max1, mut idx_max2) = (0, 1);
        for v in nums.iter() {
            if *v > max1 {
                max2 = max1;
                max1 = *v;
            } else if *v > max2 {
                max2 = *v;
            }
        }

        (max1 - 1) * (max2 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_1() {
        let nums = vec![3, 4, 5, 2];

        assert_eq!(Solution::max_product(nums), 12)
    }

    #[test]
    fn test_max_product_2() {
        let nums = vec![1, 5, 4, 5];

        assert_eq!(Solution::max_product(nums), 16)
    }

    #[test]
    fn test_max_product_3() {
        let nums = vec![3, 7];

        assert_eq!(Solution::max_product(nums), 12)
    }

    #[test]
    fn test_max_product_4() {
        let nums = vec![10, 2, 5, 2];

        assert_eq!(Solution::max_product(nums), 36)
    }
}
