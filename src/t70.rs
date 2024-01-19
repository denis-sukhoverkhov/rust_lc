// 70. Climbing Stairs
// https://leetcode.com/problems/climbing-stairs

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 0);

        for _ in 0..n {
            (a, b) = (a + b, a);
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs_1() {
        let n = 2;
        assert_eq!(Solution::climb_stairs(n), 2);
    }

    #[test]
    fn test_climb_stairs_2() {
        let n = 3;
        assert_eq!(Solution::climb_stairs(n), 3);
    }

    #[test]
    fn test_climb_stairs_3() {
        let n = 5;
        assert_eq!(Solution::climb_stairs(n), 8);
    }
}
