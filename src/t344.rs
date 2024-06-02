// 344. Reverse String
// https://leetcode.com/problems/reverse-string/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_string(s: &mut [char]) {
        let (mut l, mut r) = (0, s.len() - 1);

        while l < r {
            (s[l], s[r]) = (s[r], s[l]);

            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);

        let expected = vec!['o', 'l', 'l', 'e', 'h'];
        assert_eq!(s, expected);
    }
}
