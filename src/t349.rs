// 349. Intersection of Two Arrays
// https://leetcode.com/problems/intersection-of-two-arrays/

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1: HashSet<&i32> = HashSet::from_iter(nums1.iter());
        let s2: HashSet<&i32> = HashSet::from_iter(nums2.iter());

        let s3 = s1.intersection(&s2);

        s3.map(|&&n| n).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];

        let expected = vec![2];

        assert_eq!(Solution::intersection(nums1, nums2), expected);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let expected = vec![4, 9];

        let mut res = Solution::intersection(nums1, nums2);
        res.sort();

        assert_eq!(res, expected);
    }
}
