// 1614. Maximum Nesting Depth of the Parentheses
// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_depth(s: String) -> i32 {
        let mut res = 0;

        let mut stack = vec![];
        for c in s.chars() {
            if c == '(' {
                stack.push(c);
            } else if c == ')' {
                stack.pop();
            }

            res = res.max(stack.len() as i32)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth_1() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();

        assert_eq!(Solution::max_depth(s), 3)
    }
}
