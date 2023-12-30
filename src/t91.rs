// 91. Decode Ways
// https://leetcode.com/problems/decode-ways

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_decodings(s: String) -> i32 {
        let chars = s.chars().count();
        let mut dp = vec![1; chars + 1];

        for i in (0..chars).rev() {
            let ch = s.chars().nth(i).unwrap();
            if ch == '0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1];
            }

            if i + 1 >= chars {
                continue;
            }

            let next_ch = s.chars().nth(i + 1).unwrap();
            if "2".contains(ch) && "0123456".contains(next_ch)
                || "1".contains(ch) && "0123456789".contains(next_ch)
            {
                dp[i] += dp[i + 2];
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings_1() {
        let s = String::from("12");
        assert_eq!(Solution::num_decodings(s), 2);
    }

    #[test]
    fn test_num_decodings_2() {
        let s = String::from("226");
        assert_eq!(Solution::num_decodings(s), 3);
    }

    #[test]
    fn test_num_decodings_3() {
        let s = String::from("06");
        assert_eq!(Solution::num_decodings(s), 0);
    }

    #[test]
    fn test_num_decodings_4() {
        let s = String::from("2611055971756562");
        assert_eq!(Solution::num_decodings(s), 4);
    }
}
