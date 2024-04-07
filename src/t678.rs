// 678. Valid Parenthesis String
// https://leetcode.com/problems/valid-parenthesis-string/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn check_valid_string(s: String) -> bool {
        let mut stack_braces = vec![];
        let mut stack_asterisks = vec![];

        for (idx, c) in s.chars().enumerate() {
            if c == ')' && stack_braces.len() == 0 && stack_asterisks.len() == 0 {
                return false;
            }

            if c == '(' {
                stack_braces.push(idx);
                continue;
            }

            if c == '*' {
                stack_asterisks.push(idx);
                continue;
            }

            if c == ')' && stack_braces.len() > 0 {
                stack_braces.pop();
                continue;
            }

            if c == ')' && stack_asterisks.len() > 0 {
                stack_asterisks.pop();
                continue;
            }
        }

        loop {
            let head_braces = stack_braces.pop();
            let head_asterisks = stack_asterisks.pop();

            if let Some(idx) = head_braces {
                if let Some(idx2) = head_asterisks {
                    if idx > idx2 {
                        stack_braces.push(idx);
                        return false;
                    }
                } else {
                    stack_braces.push(idx);
                    break;
                }
            } else {
                break;
            }
        }

        if !stack_braces.is_empty() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid_string_1() {
        let s = String::from("(*))");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_2() {
        let s = String::from("(*)");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_3() {
        let s = String::from("()");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_4() {
        let s = String::from(")");
        assert!(!Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_5() {
        let s = String::from("*)");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_6() {
        let s = String::from("(*)))");
        assert!(!Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_7() {
        let s = String::from("(*");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_8() {
        let s = String::from("(*)*)*)");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_9() {
        let s = String::from("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())");
        assert!(!Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_10() {
        let s = String::from("(((((((((((((*************");
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_11() {
        let s = String::from("(");
        assert!(!Solution::check_valid_string(s));
    }
}
