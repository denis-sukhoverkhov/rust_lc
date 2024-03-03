// 2385. Amount of Time for Binary Tree to Be Infected
// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;

impl Solution {
    #[allow(dead_code)]
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let graph = Solution::build_graph(root);
        // dbg!(&graph);

        let mut visited = HashSet::new();

        fn bfs(
            node: i32,
            start_minutes: i32,
            graph: &HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
        ) -> i32 {
            if visited.contains(&node) {
                return start_minutes - 1;
            }

            visited.insert(node);

            let mut max_minutes = start_minutes;
            for i in graph.get(&node).unwrap().iter() {
                max_minutes =
                    std::cmp::max(max_minutes, bfs(*i, start_minutes + 1, graph, visited));
            }

            max_minutes
        }

        bfs(start, 0, &graph, &mut visited)
    }

    pub fn build_graph(root: Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Vec<i32>> {
        let mut graph = HashMap::new();

        fn bfs(root: Option<Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(root) = root {
                let root = root.borrow();

                graph.entry(root.val).or_default();

                if let Some(left) = root.left.clone() {
                    let left = left.borrow();
                    graph.get_mut(&root.val).unwrap().push(left.val);
                    graph.insert(left.val, vec![root.val]);
                }

                if let Some(right) = root.right.clone() {
                    let right = right.borrow();
                    graph.get_mut(&root.val).unwrap().push(right.val);
                    graph.insert(right.val, vec![root.val]);
                }

                bfs(root.left.clone(), graph);
                bfs(root.right.clone(), graph);
            }
        }

        bfs(root, &mut graph);

        graph
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_amount_of_time_1() {
        let nodes = vec![1, 5, 3, NONE, 4, 10, 6, 9, 2];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::amount_of_time(root, 3), 4);
    }

    #[test]
    fn test_amount_of_time_2() {
        let nodes = vec![1];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::amount_of_time(root, 1), 0);
    }
}
