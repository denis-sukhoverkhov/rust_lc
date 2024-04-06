// 1249. Minimum Remove to Make Valid Parentheses
// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut ct = 0;

        let mut tmp = vec![];

        for c in s.chars() {
            if c == ')' && ct == 0 {
                continue;
            }

            if c == ')' {
                ct -= 1;
            }

            if c == '(' {
                ct += 1;
            }

            tmp.push(c);
        }

        let mut res = vec![];
        for i in (0..tmp.len()).rev() {
            if ct > 0 && tmp[i] == '(' {
                ct -= 1;
            } else {
                res.push(tmp[i])
            }
        }

        res.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_remove_to_make_valid_1() {
        let s = "lee(t(c)o)de)".to_string();
        let expected = "lee(t(c)o)de".to_string();

        assert_eq!(Solution::min_remove_to_make_valid(s), expected)
    }

    #[test]
    fn test_min_remove_to_make_valid_2() {
        let s = "a)b(c)d".to_string();
        let expected = "ab(c)d".to_string();

        assert_eq!(Solution::min_remove_to_make_valid(s), expected)
    }

    #[test]
    fn test_min_remove_to_make_valid_3() {
        let s = "))((".to_string();
        let expected = "".to_string();

        assert_eq!(Solution::min_remove_to_make_valid(s), expected)
    }
}
