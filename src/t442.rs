// 442. Find All Duplicates in an Array
// https://leetcode.com/problems/find-all-duplicates-in-an-array/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        for i in 0..nums.len() {
            let number = nums[i].abs();
            let idx = number - 1;

            if nums[idx as usize] < 0 {
                res.push(number);
                continue;
            }

            nums[idx as usize] *= -1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];

        let expected = vec![2, 3];

        assert_eq!(Solution::find_duplicates(nums), expected);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![1, 1, 2];

        let expected = vec![1];

        assert_eq!(Solution::find_duplicates(nums), expected);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1];

        let expected = vec![];

        assert_eq!(Solution::find_duplicates(nums), expected);
    }
}
