// 150. Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation

#[derive(Default)]
struct Solution;

use std::ops::{Add, Div, Mul, Sub};

impl Solution {
    #[allow(dead_code)]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut st = vec![];

        for t in tokens.iter() {
            match t.as_ref() {
                "+" | "-" | "*" | "/" => {
                    let a = st.pop().unwrap();
                    let b = st.pop().unwrap();

                    let op = match t.as_ref() {
                        "+" => i32::add,
                        "-" => i32::sub,
                        "*" => i32::mul,
                        _ => i32::div,
                    };

                    st.push(op(b, a));
                }
                num => {
                    st.push(num.parse::<i32>().unwrap());
                }
            }
        }

        *st.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn_1() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];

        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn test_eval_rpn_2() {
        let tokens = vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ];

        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn test_eval_rpn_3() {
        let tokens = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];

        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
