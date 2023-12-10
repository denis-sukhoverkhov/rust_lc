// 867. Transpose Matrix
// https://leetcode.com/problems/transpose-matrix/description/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix[0].len()];

        let rows = matrix.len();
        let columns = matrix[0].len();

        (0..rows).for_each(|i| {
            (0..columns).for_each(|j| {
                res[j][i] = matrix[i][j];
            });
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(Solution::transpose(matrix), expected);
    }

    #[test]
    fn test_transpose_2() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(matrix), expected);
    }
}
