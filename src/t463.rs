// 463. Island Perimeter
// https://leetcode.com/problems/island-perimeter/

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &Vec<Vec<i32>>, i: i32, j: i32, visited: &mut HashSet<(i32, i32)>) -> i32 {
            let rows = grid.len() as i32;
            let columns = grid[0].len() as i32;
            if i < 0 || i >= rows || j < 0 || j >= columns || grid[i as usize][j as usize] == 0 {
                return 1;
            }

            let key = (i, j);
            if visited.contains(&key) {
                return 0;
            }

            visited.insert(key);

            dfs(grid, i + 1, j, visited)
                + dfs(grid, i - 1, j, visited)
                + dfs(grid, i, j + 1, visited)
                + dfs(grid, i, j - 1, visited)
        }

        let mut visited = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    return dfs(&grid, i as i32, j as i32, &mut visited);
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_island_perimeter_1() {
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(Solution::island_perimeter(grid), 16);
    }

    #[test]
    fn test_island_perimeter_2() {
        let grid = vec![vec![1]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }

    #[test]
    fn test_island_perimeter_3() {
        let grid = vec![vec![1, 0]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }
}
