// 621. Task Scheduler
// https://leetcode.com/problems/task-scheduler/

use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq_map = HashMap::new();
        for task in tasks.iter() {
            *freq_map.entry(task).or_insert(0) += 1;
        }

        let mut max_heap = BinaryHeap::new();
        for (_, freq) in freq_map.iter() {
            max_heap.push(*freq);
        }

        let mut wait_deq = VecDeque::new();
        let mut time = 0;
        while !max_heap.is_empty() || !wait_deq.is_empty() {
            if let Some(mut task) = max_heap.pop() {
                task -= 1;

                if task != 0 {
                    wait_deq.push_back((task, time + n))
                }
            }

            while !wait_deq.is_empty() && wait_deq.front().unwrap().1 == time {
                let ready_task = wait_deq.pop_front().unwrap();
                max_heap.push(ready_task.0)
            }

            time += 1;
        }

        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_interval_1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 2;

        assert_eq!(Solution::least_interval(tasks, n), 8);
    }

    #[test]
    fn test_least_interval_2() {
        let tasks = vec!['A', 'C', 'A', 'B', 'D', 'B'];
        let n = 1;

        assert_eq!(Solution::least_interval(tasks, n), 6);
    }

    #[test]
    fn test_least_interval_3() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 3;

        assert_eq!(Solution::least_interval(tasks, n), 10);
    }
}
