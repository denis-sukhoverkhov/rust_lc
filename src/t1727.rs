// 1727. Largest Submatrix With Rearrangements
// https://leetcode.com/problems/largest-submatrix-with-rearrangements/description/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        if matrix.is_empty() {
            return ans;
        }

        let mut tmp_matrix = matrix.clone();

        for j in 0..tmp_matrix[0].len() {
            let mut dp = 0;

            for row in &mut tmp_matrix {
                if row[j] == 0 {
                    dp = 0;
                } else {
                    dp += 1;
                }
                row[j] = dp;
            }
        }

        for row in &mut tmp_matrix {
            row.sort_by(|a, b| b.cmp(a));
            let mut max_row_ans = 0;
            for (j, col) in row.iter().enumerate() {
                max_row_ans = max_row_ans.max(col * (j + 1) as i32);
            }
            ans = ans.max(max_row_ans);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_submatrix_empty() {
        let matrix = vec![];

        assert_eq!(Solution::largest_submatrix(matrix), 0);
    }

    #[test]
    fn test_largest_submatrix_1() {
        let matrix = vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]];

        assert_eq!(Solution::largest_submatrix(matrix), 4);
    }

    #[test]
    fn test_largest_submatrix_2() {
        let matrix = vec![
            vec![0, 1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 1, 1, 0, 0],
            vec![0, 1, 1, 0, 1, 1, 1],
        ];

        assert_eq!(Solution::largest_submatrix(matrix), 5);
    }
}
