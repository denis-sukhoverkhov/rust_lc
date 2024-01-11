// 1026. Maximum Difference Between Node and Ancestor
// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor

#[derive(Default)]
struct Solution;

use crate::helpers::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(dead_code)]
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diff = 0;

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_diff: &mut i32) -> Option<(i32, i32)> {
            if let Some(root) = root {
                let root = root.borrow();

                let (lmin, lmax) = if let Some((min, max)) = dfs(root.left.clone(), max_diff) {
                    (min, max)
                } else {
                    (root.val, root.val)
                };

                let (rmin, rmax) = if let Some((min, max)) = dfs(root.right.clone(), max_diff) {
                    (min, max)
                } else {
                    (root.val, root.val)
                };

                let (min, max) = (lmin.min(rmin), lmax.max(rmax));

                *max_diff = *max_diff
                    .max(&mut (root.val - min).abs())
                    .max(&mut (root.val - max).abs());

                return Some((min.min(root.val), max.max(root.val)));
            }

            None
        }

        dfs(root, &mut max_diff);

        max_diff
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::{build_tree_from_vec, NONE};

    use super::*;

    #[test]
    fn test_max_ancestor_diff_1() {
        let root = build_tree_from_vec(vec![8, 3, 10, 1, 6, NONE, 14, NONE, NONE, 4, 7, 13]);
        assert_eq!(Solution::max_ancestor_diff(root), 7);
    }

    #[test]
    fn test_max_ancestor_diff_2() {
        let root = build_tree_from_vec(vec![1, NONE, 2, NONE, 0, 3]);
        assert_eq!(Solution::max_ancestor_diff(root), 3);
    }

    #[test]
    fn test_max_ancestor_diff_3() {
        let root = build_tree_from_vec(vec![
            7, 5, 12, 11, 0, 2, NONE, 4, 17, 6, 19, NONE, 16, 18, NONE, NONE, NONE, NONE, 9, 14,
            10, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, 3, 1, NONE, NONE, 8, NONE, 13,
            NONE, NONE, 15,
        ]);
        assert_eq!(Solution::max_ancestor_diff(root), 19);
    }
}
