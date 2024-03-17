// 452. Minimum Number of Arrows to Burst Balloons
// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        let mut p = 0;

        points.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut res = 0;
        while p < len - 1 {
            let start = points[p + 1][0];
            if start >= points[p][0] && start <= points[p][1] {
                points[p + 1][0] = points[p + 1][0].max(points[p][0]);
                points[p + 1][1] = points[p + 1][1].min(points[p][1]);
            } else {
                res += 1;
            }
            p += 1;
        }

        res + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_arrow_shots_1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];

        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }

    #[test]
    fn test_find_min_arrow_shots_2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];

        assert_eq!(Solution::find_min_arrow_shots(points), 4);
    }

    #[test]
    fn test_find_min_arrow_shots_3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];

        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }
}
