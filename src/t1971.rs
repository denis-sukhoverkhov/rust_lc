// 1971. Find if Path Exists in Graph
// https://leetcode.com/problems/find-if-path-exists-in-graph/

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn valid_path(_: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut _m: HashMap<i32, Vec<i32>> = HashMap::new();

        for e in edges.iter() {
            let val = _m.entry(e[0]).or_default();
            val.push(e[1]);

            let val = _m.entry(e[1]).or_default();
            val.push(e[0]);
        }

        let mut visited = HashSet::new();
        fn dfs(m: &HashMap<i32, Vec<i32>>, v: i32, dst: i32, visited: &mut HashSet<i32>) -> bool {
            if v == dst {
                return true;
            }

            if visited.contains(&v) {
                return false;
            }

            visited.insert(v);

            let vertexes: &Vec<i32> = m.get(&v).unwrap();
            for h in vertexes.iter() {
                if dfs(m, *h, dst, visited) {
                    return true;
                }
            }

            false
        }

        dfs(&_m, source, destination, &mut visited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        assert!(Solution::valid_path(3, edges, 0, 2));
    }

    #[test]
    fn test_valid_path_2() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        assert!(!Solution::valid_path(6, edges, 0, 5));
    }
}
