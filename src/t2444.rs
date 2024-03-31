// 2444. Count Subarrays With Fixed Bounds
// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res: i64 = 0;

        let (mut start, mut last_idx_min, mut last_idx_max) = (0, -1, -1);
        for (r, v) in nums.iter().enumerate() {
            if *v < min_k || *v > max_k {
                start = (r as i32) + 1;
                continue;
            }

            if *v == min_k {
                last_idx_min = r as i32;
            }

            if *v == max_k {
                last_idx_max = r as i32;
            }

            if last_idx_max >= start && last_idx_min >= start {
                let min_idx = last_idx_max.min(last_idx_min);
                res += (min_idx - start + 1) as i64;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays_1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        assert_eq!(Solution::count_subarrays(nums, 1, 5), 2);
    }

    #[test]
    fn test_count_subarrays_2() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(Solution::count_subarrays(nums, 1, 1), 10);
    }
}
