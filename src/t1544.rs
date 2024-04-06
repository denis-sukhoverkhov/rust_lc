// 1544. Make The String Great
// https://leetcode.com/problems/make-the-string-great/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn make_good(s: String) -> String {
        let mut stack = vec!['0'];
        for c in s.chars() {
            let head = stack.last().unwrap();
            let current: char = c.to_lowercase().next().unwrap();
            let head_lower = stack.last().unwrap().to_lowercase().next().unwrap();
            if head_lower == current
                && (c.is_ascii_uppercase() && head.is_ascii_lowercase()
                    || c.is_ascii_lowercase() && head.is_ascii_uppercase())
            {
                stack.pop();
                continue;
            }

            stack.push(c);
        }

        stack.into_iter().skip(1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_good_1() {
        let s = "leEeetcode".to_string();

        let expected = "leetcode".to_string();
        assert_eq!(Solution::make_good(s), expected)
    }

    #[test]
    fn test_make_good_2() {
        let s = "abBAcC".to_string();

        let expected = "".to_string();
        assert_eq!(Solution::make_good(s), expected)
    }

    #[test]
    fn test_make_good_3() {
        let s = "s".to_string();

        let expected = "s".to_string();
        assert_eq!(Solution::make_good(s), expected)
    }

    #[test]
    fn test_make_good_4() {
        let s = "Pp".to_string();

        let expected = "".to_string();
        assert_eq!(Solution::make_good(s), expected)
    }
}
