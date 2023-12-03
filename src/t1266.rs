// 1266. Minimum Time Visiting All Points
// https://leetcode.com/problems/minimum-time-visiting-all-points/description/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for v in points.windows(2) {
            let (x1, y1) = (v[0][0], v[0][1]);
            let (x2, y2) = (v[1][0], v[1][1]);

            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();

            res += dx.max(dy);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time_to_visit_all_points_1() {
        let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];

        assert_eq!(Solution::min_time_to_visit_all_points(points), 7)
    }

    #[test]
    fn test_min_time_to_visit_all_points_2() {
        let points = vec![vec![3, 2], vec![-2, 2]];

        assert_eq!(Solution::min_time_to_visit_all_points(points), 5)
    }

    #[test]
    fn test_min_time_to_visit_all_points_3() {
        let points = vec![
            vec![0, 0],
            vec![2, 2],
            vec![5, 2],
            vec![8, 5],
            vec![0, -4],
            vec![-5, 4],
        ];

        assert_eq!(Solution::min_time_to_visit_all_points(points), 25)
    }
}
