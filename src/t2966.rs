// 2966. Divide Array Into Arrays With Max Difference
// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference
#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();

        let mut res = vec![];
        for w in nums.windows(3).step_by(3) {
            if w[2] - w[0] <= k {
                res.push(vec![w[0], w[1], w[2]])
            } else {
                return vec![];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_array_1() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let expected = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];

        assert_eq!(Solution::divide_array(nums, 2), expected);
    }

    #[test]
    fn test_divide_array_2() {
        let nums = vec![1, 3, 3, 2, 7, 3];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::divide_array(nums, 3), expected);
    }
}
