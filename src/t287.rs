// 287. Find the Duplicate Number
// https://leetcode.com/problems/find-the-duplicate-number/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut s, mut f) = (0, 0);

        loop {
            s = nums[s as usize];
            f = nums[nums[f as usize] as usize];

            if s == f {
                break;
            }
        }

        let mut s2 = 0;

        loop {
            s = nums[s as usize];
            s2 = nums[s2 as usize];

            if s == s2 {
                return s;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_1() {
        let nums = vec![1, 3, 4, 2, 2];

        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    #[test]
    fn test_find_duplicate_2() {
        let nums = vec![3, 1, 3, 4, 2];

        assert_eq!(Solution::find_duplicate(nums), 3);
    }
}
