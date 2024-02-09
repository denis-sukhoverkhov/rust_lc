// 368. Largest Divisible Subset
// https://leetcode.com/problems/largest-divisible-subset

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let mut dp = vec![vec![]; nums.len()];

        let mut answ = vec![];
        for i in (0..nums.len()).rev() {
            let mut curr = vec![nums[i]];
            for j in i + 1..nums.len() {
                if nums[j] % nums[i] == 0 {
                    let mut tmp = dp[j].clone();
                    tmp.push(nums[i]);

                    curr = if tmp.len() > curr.len() { tmp } else { curr };
                }
            }

            dp[i] = curr;

            answ = if dp[i].len() > answ.len() {
                dp[i].clone()
            } else {
                answ
            }
        }

        answ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_divisible_subset_1() {
        let nums = vec![1, 2, 3];
        let expected = vec![2, 1];

        assert_eq!(Solution::largest_divisible_subset(nums), expected);
    }

    #[test]
    fn test_largest_divisible_subset_2() {
        let nums = vec![1, 2, 4, 8];
        let expected = vec![8, 4, 2, 1];

        assert_eq!(Solution::largest_divisible_subset(nums), expected);
    }
}
