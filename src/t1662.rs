// 1662. Check If Two String Arrays are Equivalent
// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/description/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_strings_are_equal_1() {
        let word1 = vec!["ab".to_string(), "c".to_string()];
        let word2 = vec!["a".to_string(), "bc".to_string()];

        assert!(Solution::array_strings_are_equal(word1, word2))
    }

    #[test]
    fn test_array_strings_are_equal_2() {
        let word1 = vec!["a".to_string(), "cb".to_string()];
        let word2 = vec!["ab".to_string(), "c".to_string()];

        assert!(!Solution::array_strings_are_equal(word1, word2))
    }

    #[test]
    fn test_array_strings_are_equal_3() {
        let word1 = vec!["abc".to_string(), "d".to_string(), "defg".to_string()];
        let word2 = vec!["abcddefg".to_string()];

        assert!(Solution::array_strings_are_equal(word1, word2))
    }
}
