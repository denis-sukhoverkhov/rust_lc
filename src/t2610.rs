// 2610. Convert an Array Into a 2D Array With Conditions
// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = vec![0; 200];
        for n in nums {
            map[(n - 1) as usize] += 1;
        }

        let mut result = vec![];
        loop {
            let mut row: Vec<i32> = vec![];

            for (i, v) in map.iter_mut().enumerate() {
                if *v == 0 {
                    continue;
                }

                row.push((i + 1) as i32);
                *v -= 1;
            }

            if row.is_empty() {
                break;
            }

            result.push(row);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matrix_1() {
        let nums = vec![1, 3, 4, 1, 2, 3, 1];
        let expected = vec![vec![1, 2, 3, 4], vec![1, 3], vec![1]];

        assert_eq!(Solution::find_matrix(nums), expected);
    }

    #[test]
    fn test_find_matrix_2() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![vec![1, 2, 3, 4]];

        assert_eq!(Solution::find_matrix(nums), expected);
    }
}
