// 1704. Determine if String Halves Are Alike
// https://leetcode.com/problems/determine-if-string-halves-are-alike

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn halves_are_alike(s: String) -> bool {
        let (s1, s2) = s.split_at(s.len() / 2);

        s1.chars().filter(Solution::is_vowel).count()
            == s2.chars().filter(Solution::is_vowel).count()
    }

    pub fn is_vowel(ch: &char) -> bool {
        matches!(
            *ch,
            'e' | 'u' | 'i' | 'o' | 'a' | 'E' | 'U' | 'I' | 'O' | 'A'
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halves_are_alike_1() {
        assert!(Solution::halves_are_alike("book".to_string()));
    }

    #[test]
    fn test_halves_are_alike_2() {
        assert!(!Solution::halves_are_alike("textbook".to_string()));
    }
}
