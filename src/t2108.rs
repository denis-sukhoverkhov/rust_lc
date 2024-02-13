// 2108. Find First Palindromic String in the Array
// https://leetcode.com/problems/find-first-palindromic-string-in-the-array

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_palindrome(words: Vec<String>) -> String {
        for w in words {
            let (mut l, mut r) = (0, w.len() - 1);

            while l < r {
                let lc = w.chars().nth(l);
                let rc = w.chars().nth(r);

                if lc != rc {
                    break;
                }

                l += 1;
                r -= 1;
            }

            if l >= r {
                return w;
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_beams_1() {
        let words = vec![
            "abc".to_string(),
            "car".to_string(),
            "ada".to_string(),
            "racecar".to_string(),
            "cool".to_string(),
        ];
        assert_eq!(Solution::first_palindrome(words), "ada".to_string());
    }

    #[test]
    fn test_number_of_beams_2() {
        let words = vec!["notapalindrome".to_string(), "racecar".to_string()];
        assert_eq!(Solution::first_palindrome(words), "racecar".to_string());
    }

    #[test]
    fn test_number_of_beams_3() {
        let words = vec!["def".to_string(), "ghi".to_string()];
        assert_eq!(Solution::first_palindrome(words), "".to_string());
    }
}
