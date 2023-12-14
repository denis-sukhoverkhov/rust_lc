// 2482. Difference Between Ones and Zeros in Row and Column
// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let ct_rows = grid.len();
        let ct_columns = grid[0].len();
        let mut ones_rows_ct = vec![0; ct_rows];
        let mut ones_cols_ct = vec![0; ct_columns];

        for (i, row) in grid.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if *v == 1 {
                    ones_rows_ct[i] += 1;
                    ones_cols_ct[j] += 1;
                }
            }
        }

        let total_elems = (ct_columns + ct_rows) as i32;

        let mut result: Vec<Vec<i32>> = vec![vec![0; ct_columns]; ct_rows];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().enumerate() {
                let total_ones = ones_rows_ct[i] + ones_cols_ct[j];
                *v = total_ones - (total_elems - total_ones);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones_minus_zeros_1() {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let should_be = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        assert_eq!(Solution::ones_minus_zeros(grid), should_be);
    }

    #[test]
    fn test_ones_minus_zeros_2() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let should_be = vec![vec![5, 5, 5], vec![5, 5, 5]];
        assert_eq!(Solution::ones_minus_zeros(grid), should_be);
    }
}
