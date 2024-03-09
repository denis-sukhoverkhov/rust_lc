// 2540. Minimum Common Value
// https://leetcode.com/problems/minimum-common-value/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut p1: i32 = 0;
        let mut p2: i32 = 0;

        let ln1 = nums1.len() as i32;
        let ln2 = nums2.len() as i32;

        while p1 < ln1 && p2 < ln2 {
            if nums1[p1 as usize] == nums2[p2 as usize] {
                return nums1[p1 as usize];
            }

            if nums1[p1 as usize] < nums2[p2 as usize] {
                p1 += 1;
            } else {
                p2 += 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_common_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];

        assert_eq!(Solution::get_common(nums1, nums2), 2);
    }

    #[test]
    fn test_get_common_2() {
        let nums1 = vec![1, 2, 3, 6];
        let nums2 = vec![2, 3, 4, 5];

        assert_eq!(Solution::get_common(nums1, nums2), 2);
    }
}
