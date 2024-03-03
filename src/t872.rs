// 872. Leaf-Similar Trees
// https://leetcode.com/problems/leaf-similar-trees

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn get_leafs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            if let Some(root) = root {
                let root = root.borrow();

                let res = if root.left.is_none() && root.right.is_none() {
                    vec![root.val]
                } else {
                    let left_leafs = get_leafs(root.left.clone());
                    let right_leafs = get_leafs(root.right.clone());

                    [left_leafs, right_leafs].concat()
                };
                return res;
            }

            vec![]
        }

        let root1_leafs = get_leafs(root1);
        let root2_leafs = get_leafs(root2);

        root1_leafs == root2_leafs
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_leaf_similar_1() {
        let root1 = TreeNode::build_tree_from_vec(vec![3, 5, 1, 6, 2, 9, 8, NONE, NONE, 7, 4]);
        let root2 = TreeNode::build_tree_from_vec(vec![
            3, 5, 1, 6, 7, 4, 2, NONE, NONE, NONE, NONE, NONE, NONE, 9, 8,
        ]);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_leaf_similar_2() {
        let root1 = TreeNode::build_tree_from_vec(vec![1, 2, 3]);
        let root2 = TreeNode::build_tree_from_vec(vec![1, 3, 2]);
        assert!(!Solution::leaf_similar(root1, root2));
    }
}
