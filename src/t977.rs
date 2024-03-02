// 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut last_elem: i32 = (nums.len() - 1) as i32;

        let (mut p1, mut p2) = (0, last_elem);
        while p1 <= p2 {
            let sq_p1 = nums[p1 as usize].pow(2);
            let sq_p2 = nums[p2 as usize].pow(2);

            if sq_p1 >= sq_p2 {
                result[last_elem as usize] = sq_p1;
                p1 += 1;
            } else {
                result[last_elem as usize] = sq_p2;
                p2 -= 1;
            }

            last_elem -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares_1() {
        let nums = vec![-4, -1, 0, 3, 10];

        assert_eq!(Solution::sorted_squares(nums), [0, 1, 9, 16, 100]);
    }

    #[test]
    fn test_sorted_squares_2() {
        let nums = vec![-7, -3, 2, 3, 11];

        assert_eq!(Solution::sorted_squares(nums), [4, 9, 9, 49, 121]);
    }

    #[test]
    fn test_sorted_squares_3() {
        let nums = vec![-7, -3, -1];

        assert_eq!(Solution::sorted_squares(nums), [1, 9, 49]);
    }

    #[test]
    fn test_sorted_squares_4() {
        let nums = vec![1, 3, 4];

        assert_eq!(Solution::sorted_squares(nums), [1, 9, 16]);
    }

    #[test]
    fn test_sorted_squares_5() {
        let nums = vec![-3];

        assert_eq!(Solution::sorted_squares(nums), [9]);
    }
}
