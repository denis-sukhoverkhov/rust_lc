// 1291. Sequential Digits
// https://leetcode.com/problems/sequential-digits/description/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let len_low = low.checked_ilog10().unwrap_or(0) as usize + 1;
        let len_high = high.checked_ilog10().unwrap_or(0) as usize + 1;

        let res = (len_low..=len_high).fold(vec![], |mut v, i| {
            let nums = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

            let mut val = nums
                .windows(i)
                .map(|a| a.iter().collect::<String>().parse::<i32>().unwrap())
                .filter(|x| x >= &low && x <= &high)
                .collect::<Vec<i32>>();

            v.append(&mut val);

            v
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_digits_1() {
        let low = 100;
        let high = 300;

        let expected = vec![123, 234];
        assert_eq!(Solution::sequential_digits(low, high), expected);
    }

    #[test]
    fn test_sequential_digits_2() {
        let low = 1000;
        let high = 13000;

        let expected = vec![1234, 2345, 3456, 4567, 5678, 6789, 12345];
        assert_eq!(Solution::sequential_digits(low, high), expected);
    }
}
