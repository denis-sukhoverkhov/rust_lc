// 513. Find Bottom Left Tree Value
// https://leetcode.com/problems/find-bottom-left-tree-value

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> (i32, i32) {
            if root.is_none() {
                return (-1, -1);
            }

            let n = root.unwrap();
            let node = n.borrow();

            let mut res = [
                (node.val, level),
                dfs(node.left.clone(), level + 1),
                dfs(node.right.clone(), level + 1),
            ];
            res.sort_by(|a, b| b.1.cmp(&a.1));
            let (val, lev) = res[0];

            (val, lev)
        }

        dfs(root, 0).0
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::{build_tree_from_vec, NONE};

    use super::*;

    #[test]
    fn test_find_bottom_left_value_1() {
        let root = build_tree_from_vec(vec![2, 1, 3]);
        assert_eq!(Solution::find_bottom_left_value(root), 1);
    }

    #[test]
    fn test_find_bottom_left_value_2() {
        let root = build_tree_from_vec(vec![1, 2, 3, 4, NONE, 5, 6, NONE, NONE, 7]);
        assert_eq!(Solution::find_bottom_left_value(root), 7);
    }
}
