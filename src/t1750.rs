// 1750. Minimum Length of String After Deleting Similar Ends
// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_length(s: String) -> i32 {
        let (mut l, mut r) = (0_i32, (s.len() - 1) as i32);

        let s = s.as_bytes();

        while l < r && s[l as usize] == s[r as usize] {
            let c = s[l as usize];

            while l <= r && s[l as usize] == c {
                l += 1;
            }

            while r >= l && s[r as usize] == c {
                r -= 1;
            }
        }

        r - l + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_length_0() {
        let s = String::from("aa");

        assert_eq!(Solution::minimum_length(s), 0);
    }

    #[test]
    fn test_minimum_length_1() {
        let s = String::from("ca");

        assert_eq!(Solution::minimum_length(s), 2);
    }

    #[test]
    fn test_minimum_length_2() {
        let s = String::from("cabaabac");

        assert_eq!(Solution::minimum_length(s), 0);
    }

    #[test]
    fn test_minimum_length_3() {
        let s = String::from("abcde");

        assert_eq!(Solution::minimum_length(s), 5);
    }

    #[test]
    fn test_minimum_length_4() {
        let s = String::from("aba");

        assert_eq!(Solution::minimum_length(s), 1);
    }
}
