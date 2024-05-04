// 881. Boats to Save People
// https://leetcode.com/problems/boats-to-save-people/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();

        let mut number_boats = 0;
        let (mut l, mut r) = (0, (people.len() - 1) as i32);
        while l <= r {
            let diff = limit - people[r as usize];
            r -= 1;
            number_boats += 1;

            if l <= r && diff >= people[l as usize] {
                l += 1;
            }
        }

        number_boats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rescue_boats_1() {
        let nums = vec![1, 2];
        let limit = 3;

        assert_eq!(Solution::num_rescue_boats(nums, limit), 1);
    }

    #[test]
    fn test_num_rescue_boats_2() {
        let nums = vec![3, 2, 2, 1];
        let limit = 3;

        assert_eq!(Solution::num_rescue_boats(nums, limit), 3);
    }

    #[test]
    fn test_num_rescue_boats_3() {
        let nums = vec![3, 5, 3, 4];
        let limit = 5;

        assert_eq!(Solution::num_rescue_boats(nums, limit), 4);
    }
}
