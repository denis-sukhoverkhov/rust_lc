// 1642. Furthest Building You Can Reach
// https://leetcode.com/problems/furthest-building-you-can-reach

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();

        for i in 1..heights.len() {
            let diff = heights[i] - heights[i - 1];

            if diff <= 0 {
                continue;
            }

            min_heap.push(Reverse(diff));
            while min_heap.len() as i32 > ladders {
                let val = min_heap.pop().unwrap();

                bricks -= val.0;

                if bricks < 0 {
                    return (i - 1) as i32;
                }
            }
        }

        (heights.len() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_building_1() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        let bricks = 5;
        let ladders = 1;

        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 4)
    }

    #[test]
    fn test_furthest_building_2() {
        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let bricks = 10;
        let ladders = 2;

        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 7)
    }

    #[test]
    fn test_furthest_building_3() {
        let heights = vec![14, 3, 19, 3];
        let bricks = 17;
        let ladders = 0;

        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 3)
    }
}
