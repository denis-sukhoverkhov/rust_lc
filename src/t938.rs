// 938. Range Sum of BST
// https://leetcode.com/problems/range-sum-of-bst

#[derive(Default)]
struct Solution;
use crate::helpers::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(dead_code)]
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node_refcell) = root {
            let node = node_refcell.borrow();

            let mut sum = 0;
            if node.val >= low && node.val <= high {
                sum += node.val;
            }

            if node.val > low {
                sum += Solution::range_sum_bst(node.left.clone(), low, high)
            }

            if node.val < high {
                sum += Solution::range_sum_bst(node.right.clone(), low, high)
            }

            sum
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::{build_tree_from_vec, NONE};

    use super::*;

    #[test]
    fn test_range_sum_bst_1() {
        let nodes = vec![10, 5, 15, 3, 7, NONE, 18];
        let root = build_tree_from_vec(nodes);
        assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
    }

    #[test]
    fn test_range_sum_bst_2() {
        let nodes = vec![10, 5, 15, 3, 7, 13, 18, 1, NONE, 6];
        let root = build_tree_from_vec(nodes);
        assert_eq!(Solution::range_sum_bst(root, 6, 10), 23);
    }
}
