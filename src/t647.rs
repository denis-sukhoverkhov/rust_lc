// 647. Palindromic Substrings
// https://leetcode.com/problems/palindromic-substrings

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0; // 1 + 1
                         // [a, b]
        for i in 0..s.len() {
            ans += Solution::count_pal(&s, i as i32, i as i32);
            ans += Solution::count_pal(&s, i as i32, (i + 1) as i32);
        }

        ans
    }

    pub fn count_pal(s: &str, mut l: i32, mut r: i32) -> i32 {
        let mut ans = 0;
        while l >= 0 && r < s.len() as i32 {
            if l == r {
                ans += 1;
            } else {
                let lc = s.chars().nth(l as usize).unwrap();
                let rc = s.chars().nth(r as usize).unwrap();

                if lc == rc {
                    ans += 1;
                } else {
                    break;
                }
            }

            l -= 1;
            r += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_substrings_1() {
        let s = "abc".to_string();
        assert_eq!(Solution::count_substrings(s), 3);
    }

    #[test]
    fn test_count_substrings_2() {
        let s = "aaa".to_string();
        assert_eq!(Solution::count_substrings(s), 6);
    }

    #[test]
    fn test_count_substrings_3() {
        let s = "fdsklf".to_string();
        assert_eq!(Solution::count_substrings(s), 6);
    }
}
