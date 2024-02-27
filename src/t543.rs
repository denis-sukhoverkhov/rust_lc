// 543. Diameter of Binary Tree
// https://leetcode.com/problems/diameter-of-binary-tree

#[derive(Default)]
struct Solution;
use crate::helpers::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut m = 0;

        fn d(root: Option<Rc<RefCell<TreeNode>>>, m: &mut i32) -> i32 {
            if root.is_none() {
                return 0;
            }

            let n = root.unwrap();
            let l_d = d(n.borrow().left.clone(), m);
            let r_d = d(n.borrow().right.clone(), m);

            *m = std::cmp::max(*m, l_d + r_d);

            1 + std::cmp::max(l_d, r_d)
        }

        d(root, &mut m);

        m
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::build_tree_from_vec;

    use super::*;

    #[test]
    fn test_diameter_of_binary_tree_1() {
        let root = build_tree_from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_of_binary_tree_2() {
        let root = build_tree_from_vec(vec![1, 2]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 1);
    }

    #[test]
    fn test_diameter_of_binary_tree_3() {
        let root = build_tree_from_vec(vec![1]);
        assert_eq!(Solution::diameter_of_binary_tree(root), 0);
    }
}
