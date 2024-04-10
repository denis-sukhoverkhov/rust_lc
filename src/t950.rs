// 2073. Time Needed to Buy Tickets
// https://leetcode.com/problems/reveal-cards-in-increasing-order/

use std::collections::VecDeque;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort();
        let mut res = vec![0; deck.len()];
        let mut dq = VecDeque::from_iter(0..deck.len());

        for v in deck.iter() {
            let idx = dq.pop_front().unwrap();
            res[idx] = *v;

            if !dq.is_empty() {
                let val = dq.pop_front().unwrap();
                dq.push_back(val);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares_1() {
        let deck = vec![17, 13, 11, 2, 3, 5, 7];

        assert_eq!(
            Solution::deck_revealed_increasing(deck),
            [2, 13, 3, 11, 5, 17, 7]
        );
    }
}
