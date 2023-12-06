// 1716. Calculate Money in Leetcode Bank
// https://leetcode.com/problems/calculate-money-in-leetcode-bank

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn total_money(n: i32) -> i32 {
        let mut total = 0;
        let mut week_number = 0;
        let mut d_increment = week_number;
        let mut week_day = 0;

        for _ in 0..n {
            d_increment += 1;
            week_day += 1;
            total += d_increment;

            if week_day == 7 {
                week_number += 1;
                d_increment = week_number;
                week_day = 0;
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_money_1() {
        assert_eq!(Solution::total_money(4), 10);
    }

    #[test]
    fn test_total_money_2() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn test_total_money_3() {
        assert_eq!(Solution::total_money(20), 96);
    }
}
