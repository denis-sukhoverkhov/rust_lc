// 713. Subarray Product Less Than K
// https://leetcode.com/problems/subarray-product-less-than-k/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res: i32 = 0;

        let (mut l, mut r) = (0, 0);
        let len = nums.len();

        let mut product = 1;
        while r < len {
            product *= nums[r];

            while product >= k && l <= r {
                product /= nums[l];
                l += 1;
            }

            res += r as i32 - l as i32 + 1;
            r += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarray_product_less_than_k_1() {
        let nnums = vec![10, 5, 2, 6];
        assert_eq!(Solution::num_subarray_product_less_than_k(nnums, 100), 8);
    }

    #[test]
    fn test_num_subarray_product_less_than_k_2() {
        let nnums = vec![1, 2, 3];
        assert_eq!(Solution::num_subarray_product_less_than_k(nnums, 0), 0);
    }
}
