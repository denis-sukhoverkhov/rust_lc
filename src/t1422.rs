// 1422. Maximum Score After Splitting a String
// https://leetcode.com/problems/maximum-score-after-splitting-a-string

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_score(s: String) -> i32 {
        let mut ones = 0;
        for c in s.chars() {
            if c == '1' {
                ones += 1;
            }
        }

        let mut ans = 0;

        let mut zeros = 0;

        let len = s.chars().count();

        for (i, c) in s.chars().enumerate() {
            if i == len - 1 {
                break;
            }

            if c == '0' {
                zeros += 1
            } else if c == '1' {
                ones -= 1;
            }

            ans = ans.max(zeros + ones);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score_1() {
        let s = String::from("011101");

        assert_eq!(Solution::max_score(s), 5)
    }

    #[test]
    fn test_max_score_2() {
        let s = String::from("00111");

        assert_eq!(Solution::max_score(s), 5)
    }

    #[test]
    fn test_max_score_3() {
        let s = String::from("1111");

        assert_eq!(Solution::max_score(s), 3)
    }

    #[test]
    fn test_max_score_4() {
        let s = String::from("00");

        assert_eq!(Solution::max_score(s), 1)
    }

    #[test]
    fn test_max_score_5() {
        let s = String::from("11");

        assert_eq!(Solution::max_score(s), 1)
    }
}
