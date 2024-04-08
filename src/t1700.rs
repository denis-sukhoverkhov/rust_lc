// 1700. Number of Students Unable to Eat Lunch
// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/

use std::collections::VecDeque;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut st_queue = VecDeque::from_iter(students);
        let mut sandwiches: Vec<i32> = sandwiches.into_iter().rev().collect();

        let mut last_served = 0;
        while !st_queue.is_empty() && last_served < st_queue.len() {
            let student: i32 = st_queue.pop_front().unwrap();
            let sandwich: &i32 = sandwiches.last().unwrap();

            if student == *sandwich {
                sandwiches.pop();
                last_served = 0;
            } else {
                st_queue.push_back(student);

                last_served += 1;
            }
        }

        last_served as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_students_1() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }

    #[test]
    fn test_count_students_2() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        assert_eq!(Solution::count_students(students, sandwiches), 3);
    }

    #[test]
    fn test_count_students_3() {
        let students = vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 0];
        let sandwiches = vec![1, 0, 1, 0, 0, 1, 1, 0, 0, 0];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }
}
