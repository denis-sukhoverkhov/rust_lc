// 200. Number of Islands
// https://leetcode.com/problems/number-of-islands/

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(i: i32, j: i32, grid: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>) {
            let key = (i, j);
            if i < 0
                || i >= grid.len() as i32
                || j < 0
                || j >= grid[0].len() as i32
                || visited.contains(&key)
                || grid[i as usize][j as usize] == '0'
            {
                return;
            }

            visited.insert(key);

            dfs(i + 1, j, grid, visited);
            dfs(i - 1, j, grid, visited);
            dfs(i, j + 1, grid, visited);
            dfs(i, j - 1, grid, visited);
        }

        let mut visited = HashSet::new();
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited.contains(&(i as i32, j as i32)) && grid[i][j] == '1' {
                    dfs(i as i32, j as i32, &grid, &mut visited);
                    res += 1
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_num_islands_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(Solution::num_islands(grid), 3);
    }
}
