// 931. Minimum Falling Path Sum
// https://leetcode.com/problems/minimum-falling-path-sum

use std::{cmp::min, collections::HashMap};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();

        let mut cache = HashMap::new();

        fn dfs(
            r: i32,
            c: i32,
            rows: i32,
            m: &[Vec<i32>],
            cache: &mut HashMap<(i32, i32), i32>,
        ) -> i32 {
            if r == rows {
                return 0;
            }

            if c < 0 || c >= rows {
                return i32::MAX;
            }

            let key = (r, c);
            if cache.contains_key(&key) {
                return *cache.get(&key).unwrap();
            }

            let v = m[r as usize][c as usize];

            let min = v + min(
                min(
                    dfs(r + 1, c - 1, rows, m, cache),
                    dfs(r + 1, c, rows, m, cache),
                ),
                dfs(r + 1, c + 1, rows, m, cache),
            );

            cache.insert(key, min);

            min
        }

        let mut res = i32::MAX;
        for c in 0..n {
            res = min(res, dfs(0, c as i32, n as i32, &matrix, &mut cache))
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_falling_path_sum_1() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(matrix), 13);
    }

    #[test]
    fn test_min_falling_path_sum_2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(Solution::min_falling_path_sum(matrix), -59);
    }
}
