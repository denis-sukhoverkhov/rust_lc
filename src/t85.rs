// 85. Maximal Rectangle
// https://leetcode.com/problems/maximal-rectangle/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut dp = vec![0; matrix[0].len()];
        let mut max_area = 0;

        for row in matrix.iter() {
            for (j, c_val) in row.iter().enumerate() {
                let val: i32 = c_val.to_digit(10).unwrap() as i32;
                dp[j] = if val != 0 { val + dp[j] } else { 0 };
            }

            let mut stack: Vec<(i32, i32)> = vec![];
            (0..dp.len()).for_each(|idx| {
                if dp[idx] == 0 && stack.is_empty() {
                    return;
                }

                let mut new_idx = idx as i32;
                while let Some((k, h)) = stack.pop() {
                    if dp[idx] >= h {
                        stack.push((k, h));
                        break;
                    }

                    max_area = max_area.max(h * ((idx as i32) - k));
                    new_idx = k;
                }

                if dp[idx] == 0 {
                    return;
                }

                stack.push((new_idx, dp[idx]));
            });

            // if there is something left on the stack
            for (i, h) in stack.iter() {
                max_area = max_area.max(h * (dp.len() as i32 - i))
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_rectangle_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];

        assert_eq!(Solution::maximal_rectangle(matrix), 6);
    }

    #[test]
    fn test_maximal_rectangle_2() {
        let matrix = vec![vec!['0']];

        assert_eq!(Solution::maximal_rectangle(matrix), 0);
    }
}
