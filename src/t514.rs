// 514. Freedom Trail
// https://leetcode.com/problems/freedom-trail

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut dp = vec![0; ring.len()];

        for c in key.chars().rev() {
            let mut new_dp = vec![i32::MAX; ring.len()];
            for (i, _) in ring.chars().enumerate() {
                for (j, v2) in ring.chars().enumerate() {
                    if v2 == c {
                        let diff = (j as i32 - i as i32).abs();
                        let curr_min = diff.min(ring.len() as i32 - diff);
                        new_dp[i] = new_dp[i].min(dp[j] + 1 + curr_min);
                    }
                }
            }

            dp = new_dp;
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_rotate_steps_1() {
        let ring = "godding".to_string();
        let key = "gd".to_string();
        assert_eq!(Solution::find_rotate_steps(ring, key), 4);
    }

    #[test]
    fn test_find_rotate_steps_2() {
        let ring = "godding".to_string();
        let key = "godding".to_string();
        assert_eq!(Solution::find_rotate_steps(ring, key), 13);
    }
}
