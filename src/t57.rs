// 57. Insert Interval
// https://leetcode.com/problems/insert-interval/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let (mut ns, mut ne) = (new_interval[0], new_interval[1]);
        let mut p = 0;

        while p < intervals.len() && intervals[p][1] < ns {
            result.push(intervals[p].clone());
            p += 1;
        }

        while p < intervals.len() && ne >= intervals[p][0] {
            ns = ns.min(intervals[p][0]);
            ne = ne.max(intervals[p][1]);
            p += 1;
        }

        result.push(vec![ns, ne]);

        while p < intervals.len() {
            result.push(intervals[p].clone());
            p += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_1() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];

        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
}
