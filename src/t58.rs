// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        let mut prev = 0;
        let mut res = 0;

        for c in s.chars() {
            if c == ' ' {
                if res != 0 {
                    prev = res;
                }
                res = 0;
                continue;
            }

            res += 1;
        }

        if res == 0 {
            res = prev;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word_1() {
        let s = "Hello World".to_string();
        assert_eq!(Solution::length_of_last_word(s), 5);
    }

    #[test]
    fn test_length_of_last_word_2() {
        let s = "   fly me   to   the moon  ".to_string();
        assert_eq!(Solution::length_of_last_word(s), 4);
    }

    #[test]
    fn test_length_of_last_word_3() {
        let s = "luffy is still joyboy".to_string();
        assert_eq!(Solution::length_of_last_word(s), 6);
    }
}
