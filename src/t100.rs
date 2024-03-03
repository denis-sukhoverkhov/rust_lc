// 100. Same Tree
// https://leetcode.com/problems/same-tree

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() && q.is_some() || q.is_none() && p.is_some() {
            return false;
        }

        let pv = p.unwrap();
        let qv = q.unwrap();
        let pv = pv.borrow();
        let qv = qv.borrow();

        if pv.val != qv.val {
            return false;
        }

        Solution::is_same_tree(pv.left.clone(), qv.left.clone())
            && Solution::is_same_tree(pv.right.clone(), qv.right.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_is_same_tree_1() {
        let p = vec![1, 2, 3];
        let q = vec![1, 2, 3];

        assert!(Solution::is_same_tree(
            TreeNode::build_tree_from_vec(p),
            TreeNode::build_tree_from_vec(q)
        ));
    }

    #[test]
    fn test_is_same_tree_2() {
        let p = vec![1, 2];
        let q = vec![1, NONE, 2];

        assert!(!Solution::is_same_tree(
            TreeNode::build_tree_from_vec(p),
            TreeNode::build_tree_from_vec(q)
        ));
    }

    #[test]
    fn test_is_same_tree_3() {
        let p = vec![1, 2, 1];
        let q = vec![1, 1, 2];

        assert!(!Solution::is_same_tree(
            TreeNode::build_tree_from_vec(p),
            TreeNode::build_tree_from_vec(q)
        ));
    }
}
