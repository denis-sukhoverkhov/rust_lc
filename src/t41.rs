// 41. First Missing Positive
// https://leetcode.com/problems/first-missing-positive/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        //first part of alghorithm for case if range doesn't have an 1
        let mut has_one = false;
        for i in nums.iter() {
            if *i == 1 {
                has_one = true;
            }
        }

        if !has_one {
            return 1;
        }

        // second part of algo
        let len = nums.len();
        for n in nums.iter_mut() {
            if *n < 1 || *n > len as i32 {
                *n = 1;
            }
        }

        for i in 0..len {
            let idx = nums[i].unsigned_abs() as usize;
            if nums[idx - 1] < 0 {
                continue;
            }
            nums[idx - 1] *= -1;
        }

        for (i, _) in nums.iter().enumerate() {
            if nums[i] > 0 {
                return (i + 1) as i32;
            }
        }

        (nums.len() + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive_1() {
        let nums = vec![1, 2, 0];

        assert_eq!(Solution::first_missing_positive(nums), 3);
    }

    #[test]
    fn test_first_missing_positive_2() {
        let nums = vec![3, 4, -1, 1];

        assert_eq!(Solution::first_missing_positive(nums), 2);
    }

    #[test]
    fn test_first_missing_positive_3() {
        let nums = vec![7, 8, 9, 11, 12];

        assert_eq!(Solution::first_missing_positive(nums), 1);
    }

    #[test]
    fn test_first_missing_positive_4() {
        let nums = vec![1];

        assert_eq!(Solution::first_missing_positive(nums), 2);
    }
}
