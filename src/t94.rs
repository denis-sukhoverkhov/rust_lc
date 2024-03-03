// 94. Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/description

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let result = vec![node.borrow().val];
                let left_nodes = Solution::inorder_traversal(node.borrow().left.clone());
                let right_nodes = Solution::inorder_traversal(node.borrow().right.clone());

                [left_nodes, result, right_nodes].concat()
            }
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_inorder_traversal_1() {
        let nodes = vec![1, NONE, 2, 3];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_inorder_traversal_2() {
        let nodes = vec![];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::inorder_traversal(root), vec![]);
    }

    #[test]
    fn test_inorder_traversal_3() {
        let nodes = vec![1];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::inorder_traversal(root), vec![1]);
    }
}
