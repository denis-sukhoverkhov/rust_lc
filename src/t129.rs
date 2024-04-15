// 129. Sum Root to Leaf Numbers
// https://leetcode.com/problems/sum-root-to-leaf-numbers/

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut storage = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, store: &mut Vec<i32>, mut num: i32) {
            if root.is_none() {
                return;
            }

            let root = root.unwrap();
            let root = root.borrow();

            num = num * 10 + root.val;

            if root.left.is_none() && root.right.is_none() {
                if num != 0 {
                    store.push(num)
                }
                return;
            }

            dfs(root.left.clone(), store, num);
            dfs(root.right.clone(), store, num);
        }

        dfs(root, &mut storage, 0);

        // dbg!(&storage);
        storage.iter().sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum_numbers_1() {
        let head = TreeNode::build_tree_from_vec(vec![1, 2, 3]);

        assert_eq!(Solution::sum_numbers(head), 25);
    }
}
