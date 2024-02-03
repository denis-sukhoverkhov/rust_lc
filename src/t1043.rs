// 1043. Partition Array for Maximum Sum
// https://leetcode.com/problems/partition-array-for-maximum-sum

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let ln = arr.len();
        let mut dp = vec![0; ln];

        for i in 0..ln {
            let mut curr_max = 0;
            let mut max_at_i = 0;

            let pos = i as i32;
            for j in ((pos - k + 1)..=pos).rev() {
                if j < 0 {
                    break;
                }

                curr_max = curr_max.max(arr[j as usize]);
                let window_size = pos - j + 1;
                let curr_sum = window_size * curr_max;
                let sub_sum = curr_sum + if j - 1 < 0 { 0 } else { dp[j as usize - 1] };

                max_at_i = max_at_i.max(sub_sum);
            }

            dp[i] = max_at_i;
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_after_partitioning_1() {
        let arr = vec![1, 15, 7, 9, 2, 5, 10];

        assert_eq!(Solution::max_sum_after_partitioning(arr, 3), 84);
    }

    #[test]
    fn test_max_sum_after_partitioning_2() {
        let arr = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];

        assert_eq!(Solution::max_sum_after_partitioning(arr, 4), 83);
    }

    #[test]
    fn test_max_sum_after_partitioning_3() {
        let arr = vec![1];

        assert_eq!(Solution::max_sum_after_partitioning(arr, 1), 1);
    }
}
