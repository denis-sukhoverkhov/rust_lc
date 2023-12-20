// 2706. Buy Two Chocolates
// https://leetcode.com/problems/buy-two-chocolates/description

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut min1 = 101;
        let mut min2 = 101;

        for p in prices {
            if p < min1 {
                (min1, min2) = (p, min1);
            } else if p < min2 {
                min2 = p;
            }
        }

        if min1 + min2 > money {
            return money;
        }

        money - (min1 + min2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_choco_1() {
        let prices = vec![1, 2, 2];
        let money = 3;
        assert_eq!(Solution::buy_choco(prices, money), 0);
    }

    #[test]
    fn test_buy_choco_2() {
        let prices = vec![3, 2, 3];
        let money = 3;
        assert_eq!(Solution::buy_choco(prices, money), 3);
    }

    #[test]
    fn test_buy_choco_3() {
        let prices = vec![5, 4, 2, 3, 1];
        let money = 4;
        assert_eq!(Solution::buy_choco(prices, money), 1);
    }
}
