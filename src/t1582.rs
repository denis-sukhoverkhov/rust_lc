// 1582. Special Positions in a Binary Matrix
// https://leetcode.com/problems/special-positions-in-a-binary-matrix

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut rows_ones = HashMap::new();
        let mut columns_ones = HashMap::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if *v == 1 {
                    let r_ct = rows_ones.entry(i).or_insert(0);
                    *r_ct += 1;

                    let col_ct = columns_ones.entry(j).or_insert(0);
                    *col_ct += 1;
                }
            }
        }

        let mut num_special_positions = 0;

        for (i, row) in mat.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if *v == 1 {
                    let ct_r = rows_ones.get(&i).copied().unwrap_or(0);
                    let ct_col = columns_ones.get(&j).copied().unwrap_or(0);
                    if ct_r > 1 || ct_col > 1 {
                        continue;
                    }

                    num_special_positions += 1
                }
            }
        }

        num_special_positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_special_1() {
        let mat = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];

        assert_eq!(Solution::num_special(mat), 1)
    }

    #[test]
    fn test_num_special_2() {
        let mat = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

        assert_eq!(Solution::num_special(mat), 3)
    }
}
