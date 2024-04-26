// 2370. Longest Ideal Subsequence
// https://leetcode.com/problems/longest-ideal-subsequence/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = [0; 26];

        for c in s.chars() {
            let idx = c as u32 - 'a' as u32;

            let mut longest = 1;
            for i in 0..26 {
                if (i as i32 - idx as i32).abs() <= k {
                    longest = longest.max(1 + dp[i])
                }
            }

            dp[idx as usize] = dp[idx as usize].max(longest);
        }

        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_ideal_string_1() {
        let s = "acfgbd".to_string();
        assert_eq!(Solution::longest_ideal_string(s, 2), 4);
    }

    #[test]
    fn test_longest_ideal_string_2() {
        let s = "abcd".to_string();
        assert_eq!(Solution::longest_ideal_string(s, 3), 4);
    }
}
