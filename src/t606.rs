// 606. Construct String from Binary Tree
// https://leetcode.com/problems/construct-string-from-binary-tree/description
use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(node) => {
                let mut right_part = Solution::tree2str(node.borrow().right.clone());
                if !right_part.is_empty() {
                    right_part = format!("({})", right_part);
                }

                let mut left_part = Solution::tree2str(node.borrow().left.clone());
                if !left_part.is_empty() || !right_part.is_empty() {
                    left_part = format!("({})", left_part);
                }

                format!("{}{}{}", node.borrow().val, left_part, right_part)
            }
            None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_tree2str_1() {
        let nodes = vec![1, 2, 3, 4];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::tree2str(root), "1(2(4))(3)");
    }

    #[test]
    fn test_tree2str_2() {
        let nodes = vec![1, 2, 3, NONE, 4];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(Solution::tree2str(root), "1(2()(4))(3)");
    }
}
