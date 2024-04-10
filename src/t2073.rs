// 2073. Time Needed to Buy Tickets
// https://leetcode.com/problems/time-needed-to-buy-tickets/

use std::cmp::min;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut total_sec = 0;

        for (idx, val) in tickets.iter().enumerate() {
            if idx as i32 <= k {
                total_sec += tickets[k as usize].min(*val)
            } else {
                total_sec += min(*val, tickets[k as usize] - 1);
            }
        }

        total_sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_beams_1() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        assert_eq!(Solution::time_required_to_buy(tickets, k), 6);
    }

    #[test]
    fn test_number_of_beams_2() {
        let tickets = vec![5, 1, 1, 1];
        let k = 0;
        assert_eq!(Solution::time_required_to_buy(tickets, k), 8);
    }

    #[test]
    fn test_number_of_beams_3() {
        let tickets = vec![84, 49, 5, 24, 70, 77, 87, 8];
        let k = 3;
        assert_eq!(Solution::time_required_to_buy(tickets, k), 154);
    }
}
