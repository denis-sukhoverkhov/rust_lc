#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let double_s = format!("{}{}", s, s);

        double_s.contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string_1() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
    }

    #[test]
    fn test_rotate_string_2() {
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
    }
}
