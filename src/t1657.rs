// 1657. Determine if Two Strings Are Close
// https://leetcode.com/problems/determine-if-two-strings-are-close

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn close_strings(word1: String, word2: String) -> bool {
        let (mut map1, mut map2) = ([0; 26], [0; 26]);

        for ch in word1.chars() {
            let idx = ch as usize - 'a' as usize;
            map1[idx] += 1;
        }

        for ch in word2.chars() {
            let idx = ch as usize - 'a' as usize;
            map2[idx] += 1;
        }

        for i in 0..26 {
            if map1[i] == 0 && map2[i] != 0 || map1[i] != 0 && map2[i] == 0 {
                return false;
            }
        }

        map1.sort();
        map2.sort();

        map1 == map2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_strings_1() {
        let word1 = "abc".to_string();
        let word2 = "bca".to_string();

        assert!(Solution::close_strings(word1, word2))
    }

    #[test]
    fn test_close_strings_2() {
        let word1 = "a".to_string();
        let word2 = "aa".to_string();

        assert!(!Solution::close_strings(word1, word2))
    }

    #[test]
    fn test_close_strings_3() {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();

        assert!(Solution::close_strings(word1, word2))
    }
}
