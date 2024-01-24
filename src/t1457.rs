// 1457. Pseudo-Palindromic Paths in a Binary Tree
// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree

#[derive(Default)]
struct Solution;

use crate::helpers::treenode::TreeNode;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    #[allow(dead_code)]
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, s: &mut HashSet<i32>) -> i32 {
            if let Some(root) = root {
                let root = root.borrow();

                if s.contains(&root.val) {
                    s.remove(&root.val);
                } else {
                    s.insert(root.val);
                }

                if root.left.is_none() && root.right.is_none() {
                    if s.len() <= 1 {
                        return 1;
                    }

                    return 0;
                }

                let mut sc1 = s.clone();
                let mut sc2 = s.clone();
                let left_ct = dfs(root.left.clone(), &mut sc1);
                let right_ct = dfs(root.right.clone(), &mut sc2);

                return left_ct + right_ct;
            }

            0
        }

        let mut s: HashSet<i32> = HashSet::new();
        dfs(root, &mut s)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::{build_tree_from_vec, NONE};

    use super::*;

    #[test]
    fn test_pseudo_palindromic_paths_1() {
        let root = build_tree_from_vec(vec![2, 3, 1, 3, 1, NONE, 1]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
    }

    #[test]
    fn test_pseudo_palindromic_paths_2() {
        let root = build_tree_from_vec(vec![2, 1, 1, 1, 3, NONE, NONE, NONE, NONE, NONE, 1]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }

    #[test]
    fn test_pseudo_palindromic_paths_3() {
        let root = build_tree_from_vec(vec![9]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }

    #[test]
    fn test_pseudo_palindromic_paths_4() {
        let root = build_tree_from_vec(vec![]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 0);
    }

    #[test]
    fn test_pseudo_palindromic_paths_5() {
        let root = build_tree_from_vec(vec![
            9, 5, 4, 5, NONE, 2, 6, 2, 5, NONE, 8, 3, 9, 2, 3, 1, 1, NONE, 4, 5, 4, 2, 2, 6, 4,
            NONE, NONE, 1, 7, NONE, 5, 4, 7, NONE, NONE, 7, NONE, 1, 5, 6, 1, NONE, NONE, NONE,
            NONE, 9, 2, NONE, 9, 7, 2, 1, NONE, NONE, NONE, 6, NONE, NONE, NONE, NONE, NONE, NONE,
            NONE, NONE, NONE, 5, NONE, NONE, 3, NONE, NONE, NONE, 8, NONE, 1, NONE, NONE, 8, NONE,
            NONE, NONE, NONE, 2, NONE, 8, 7,
        ]);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }
}
