// 1496. Path Crossing
// https://leetcode.com/problems/path-crossing/

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_path_crossing(path: String) -> bool {
        let (mut x, mut y) = (0, 0);

        let mut visited = HashSet::new();
        visited.insert((0, 0));

        for c in path.chars() {
            if c == 'N' {
                x += 1;
            } else if c == 'S' {
                x -= 1;
            } else if c == 'E' {
                y += 1;
            } else {
                y -= 1;
            }

            if visited.contains(&(x, y)) {
                return true;
            }

            visited.insert((x, y));
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_path_crossing_1() {
        let path = String::from("NES");

        assert!(!Solution::is_path_crossing(path))
    }

    #[test]
    fn test_is_path_crossing_2() {
        let path = String::from("NESWW");

        assert!(Solution::is_path_crossing(path))
    }

    #[test]
    fn test_is_path_crossing_3() {
        let path = String::from("NNSWWEWSSESSWENNW");

        assert!(Solution::is_path_crossing(path))
    }
}
