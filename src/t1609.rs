// 1609. Even Odd Tree
// https://leetcode.com/problems/even-odd-tree

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut level_nums = HashMap::new();

        Solution::dfs(root, 0, &mut level_nums)
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        l: i32,
        level_nums: &mut HashMap<i32, i32>,
    ) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.unwrap();
        let root = root.borrow();

        // even level and even val
        if l % 2 == 0 && root.val % 2 == 0 {
            return false;
        }

        // odd level and odd val
        if l % 2 != 0 && root.val % 2 != 0 {
            return false;
        }

        if let Some(prev_level_val) = level_nums.get(&l) {
            if l % 2 == 0 && *prev_level_val >= root.val {
                return false;
            }

            if l % 2 != 0 && *prev_level_val <= root.val {
                return false;
            }
        };

        level_nums.insert(l, root.val);

        Solution::dfs(root.left.clone(), l + 1, level_nums)
            && Solution::dfs(root.right.clone(), l + 1, level_nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_is_even_odd_tree_1() {
        let root =
            TreeNode::build_tree_from_vec(vec![1, 10, 4, 3, NONE, 7, 9, 12, 8, 6, NONE, NONE, 2]);

        assert!(Solution::is_even_odd_tree(root))
    }

    #[test]
    fn test_is_even_odd_tree_2() {
        let root = TreeNode::build_tree_from_vec(vec![5, 4, 2, 3, 3, 7]);

        assert!(!Solution::is_even_odd_tree(root))
    }

    #[test]
    fn test_is_even_odd_tree_3() {
        let root = TreeNode::build_tree_from_vec(vec![5, 9, 1, 3, 5, 7]);

        assert!(!Solution::is_even_odd_tree(root))
    }

    #[test]
    fn test_is_even_odd_tree_4() {
        let root = TreeNode::build_tree_from_vec(vec![9]);

        assert!(Solution::is_even_odd_tree(root))
    }

    #[test]
    fn test_is_even_odd_tree_6() {
        let root = TreeNode::build_tree_from_vec(vec![4]);

        assert!(!Solution::is_even_odd_tree(root))
    }
}
