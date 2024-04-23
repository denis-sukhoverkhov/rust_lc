// 310. Minimum Height Trees
// https://leetcode.com/problems/minimum-height-trees/

use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let mut m: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges.iter() {
            let from: i32 = edge[0];
            let to: i32 = edge[1];

            let v = m.entry(from).or_default();
            v.push(to);

            let v = m.entry(to).or_default();
            v.push(from);
        }

        let mut leaves = VecDeque::new();

        let mut ct_edges = HashMap::new();
        for (src, edges) in m.iter() {
            if edges.len() == 1 {
                leaves.push_back(*src);
            }

            ct_edges.insert(src, edges.len());
        }

        while !leaves.is_empty() {
            if n <= 2 {
                return leaves.iter().cloned().collect();
            }

            for _ in 0..leaves.len() {
                let node = leaves.pop_front().unwrap();
                n -= 1;

                if let Some(rel_nodes) = m.get(&node) {
                    for rel_node in rel_nodes {
                        let val = ct_edges.get_mut(rel_node).unwrap();
                        *val -= 1;

                        if *val == 1 {
                            leaves.push_back(*rel_node);
                        }
                    }
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_height_trees_1() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];

        assert_eq!(Solution::find_min_height_trees(4, edges), [1]);
    }

    #[test]
    fn test_find_min_height_trees_2() {
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];

        let mut res = Solution::find_min_height_trees(6, edges);
        res.sort();
        assert_eq!(res, [3, 4]);
    }

    #[test]
    fn test_find_min_height_trees_3() {
        let edges = vec![];

        assert_eq!(Solution::find_min_height_trees(1, edges), [0]);
    }

    #[test]
    fn test_find_min_height_trees_4() {
        let edges = vec![vec![0, 1]];

        let mut res = Solution::find_min_height_trees(2, edges);

        res.sort();

        assert_eq!(res, [0, 1]);
    }

    #[test]
    fn test_find_min_height_trees_5() {
        let edges = vec![vec![0, 1], vec![1, 2]];

        let mut res = Solution::find_min_height_trees(3, edges);

        res.sort();

        assert_eq!(res, [1]);
    }
}
