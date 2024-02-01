// 739. Daily Temperatures
// https://leetcode.com/problems/daily-temperatures

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];

        let mut stack: Vec<(usize, i32)> = vec![];
        for (i, t) in temperatures.iter().enumerate() {
            if stack.is_empty() {
                stack.push((i, *t));
                continue;
            }

            while let Some((j, v)) = stack.pop() {
                if v < *t {
                    ans[j] = (i - j) as i32;
                } else {
                    stack.push((j, v));
                    break;
                }
            }

            stack.push((i, *t));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];

        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }

    #[test]
    fn test_daily_temperatures_2() {
        let temperatures = vec![30, 40, 50, 60];

        let expected = vec![1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }

    #[test]
    fn test_daily_temperatures_3() {
        let temperatures = vec![30, 60, 90];

        let expected = vec![1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), expected);
    }
}
