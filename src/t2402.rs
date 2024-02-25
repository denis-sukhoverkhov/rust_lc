// 2402. Meeting Rooms III
// https://leetcode.com/problems/meeting-rooms-iii

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut meetings = meetings
            .into_iter()
            .map(|m| (m[0] as i64, m[1] as i64))
            .collect::<Vec<_>>();
        meetings.sort_unstable_by_key(|m| m.0);

        let mut occupied = BinaryHeap::new(); // (end, room)
        let mut free = (0..n).map(Reverse).collect::<BinaryHeap<_>>();
        let mut count = vec![0; n];

        for (start, mut end) in meetings {
            while let Some(Reverse((t, room))) = occupied.pop() {
                if t > start {
                    occupied.push(Reverse((t, room)));
                    break;
                }
                free.push(Reverse(room));
            }

            if free.is_empty() {
                let Reverse((t, room)) = occupied.pop().unwrap();
                end = t + (end - start);
                free.push(Reverse(room));
            }

            let Reverse(room) = free.pop().unwrap();
            occupied.push(Reverse((end, room)));
            count[room] += 1;
        }
        count
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, Reverse(i)))
            .max()
            .unwrap()
            .1
             .0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_booked_1() {
        let n = 2;
        let meetings = vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]];

        assert_eq!(Solution::most_booked(n, meetings), 0);
    }

    #[test]
    fn test_most_booked_2() {
        let n = 3;
        let meetings = vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]];

        assert_eq!(Solution::most_booked(n, meetings), 1);
    }

    #[test]
    fn test_most_booked_3() {
        let n = 2;
        let meetings = vec![vec![0, 10], vec![1, 2], vec![12, 14], vec![13, 15]];

        assert_eq!(Solution::most_booked(n, meetings), 0);
    }

    #[test]
    fn test_most_booked_4() {
        let n = 2;
        let meetings = vec![
            vec![10, 11],
            vec![2, 10],
            vec![1, 17],
            vec![9, 13],
            vec![18, 20],
        ];

        assert_eq!(Solution::most_booked(n, meetings), 1);
    }

    #[test]
    fn test_most_booked_5() {
        let n = 3;
        let meetings = vec![vec![0, 10], vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6]];

        assert_eq!(Solution::most_booked(n, meetings), 1);
    }
}
