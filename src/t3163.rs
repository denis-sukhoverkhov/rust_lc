// 3163. String Compression III
// https://leetcode.com/problems/string-compression-iii/submissions/
#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn compressed_string(word: String) -> String {
        let mut result = String::new();

        let mut counter = 0;
        let mut current_char = word.chars().next().unwrap();
        for c in word.chars() {
            if c == current_char && counter < 9 {
                counter += 1;
            } else {
                result.push_str(&counter.to_string());
                result.push(current_char);

                current_char = c;
                counter = 1;
            }
        }

        result.push_str(&counter.to_string());
        result.push(current_char);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_string_1() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e"
        );
    }

    #[test]
    fn test_compressed_string_2() {
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b"
        );
    }
}
