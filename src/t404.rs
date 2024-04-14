// 404. Sum of Left Leaves
// https://leetcode.com/problems/sum-of-left-leaves/

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, is_l: bool) -> i32 {
            if root.is_none() {
                return 0;
            }

            let curr = root.unwrap();
            let curr = curr.borrow();
            if curr.left.is_none() && curr.right.is_none() && is_l {
                return curr.val;
            }

            dfs(curr.left.clone(), true) + dfs(curr.right.clone(), false)
        }

        dfs(root, false)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let root = TreeNode::build_tree_from_vec(vec![3, 9, 20, NONE, NONE, 15, 7]);

        assert_eq!(Solution::sum_of_left_leaves(root), 24);
    }

    #[test]
    fn test_find_duplicates_2() {
        let root = TreeNode::build_tree_from_vec(vec![1]);

        assert_eq!(Solution::sum_of_left_leaves(root), 0);
    }
}
