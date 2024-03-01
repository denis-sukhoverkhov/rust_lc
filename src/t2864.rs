// 2864. Maximum Odd Binary Number
// https://leetcode.com/problems/maximum-odd-binary-number/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut ct_ones = s.chars().filter(|&c| c == '1').fold(0, |acc, _| acc + 1);

        let len = s.chars().count();
        let mut result = vec!['0'; len];
        result[len - 1] = '1';
        ct_ones -= 1;

        let mut pointer: usize = 0;
        while ct_ones > 0 {
            result[pointer] = '1';
            pointer += 1;
            ct_ones -= 1;
        }

        result.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_odd_binary_number_1() {
        let s = "010".to_string();

        assert_eq!(Solution::maximum_odd_binary_number(s), "001")
    }

    #[test]
    fn test_maximum_odd_binary_number_2() {
        let s = "0101".to_string();

        assert_eq!(Solution::maximum_odd_binary_number(s), "1001")
    }
}
