// 2225. Find Players With Zero or One Losses
// https://leetcode.com/problems/find-players-with-zero-or-one-losses

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut loosers_map = HashMap::new();
        for m in matches.iter() {
            let looser = m[1];
            let lct = loosers_map.entry(looser).or_insert(0);

            *lct += 1;
        }

        // collect winners
        let mut winners = HashSet::new();
        for m in matches.iter() {
            let winner = m[0];

            if !loosers_map.contains_key(&winner) {
                winners.insert(winner);
            }
        }
        let mut winners: Vec<i32> = winners.into_iter().collect();
        winners.sort();

        //collect loosers
        let mut loosers = vec![];
        for m in matches.iter() {
            let looser = m[1];

            let looserct = loosers_map.get(&looser).unwrap_or(&0);

            if *looserct <= 1 {
                loosers.push(looser);
            }
        }
        let mut loosers: Vec<i32> = loosers.into_iter().collect();
        loosers.sort();

        vec![winners, loosers]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winners_1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];

        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];

        assert_eq!(Solution::find_winners(matches), expected);
    }

    #[test]
    fn test_find_winners_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];

        let expected = vec![vec![1, 2, 5, 6], vec![]];

        assert_eq!(Solution::find_winners(matches), expected);
    }
}
