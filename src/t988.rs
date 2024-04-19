// 988. Smallest String Starting From Leaf
// https://leetcode.com/problems/smallest-string-starting-from-leaf

#[derive(Default)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::treenode::TreeNode;
impl Solution {
    #[allow(dead_code)]
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = vec![26];

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>, curr: &mut Vec<i32>) {
            if let Some(root) = root {
                let root = root.borrow();
                curr.push(root.val);

                if root.left.is_none() && root.right.is_none() {
                    let reversed: Vec<i32> = curr.into_iter().map(|x| *x).rev().collect();

                    let mut i = 0;
                    let mut ct = 0;
                    while i < reversed.len() && i < s.len() {
                        if s[i] > reversed[i] {
                            *s = reversed.clone();
                            break;
                        }

                        if s[i] < reversed[i] {
                            break;
                        }

                        if s[i] == reversed[i] {
                            ct += 1;
                        }

                        i += 1;
                    }

                    let max_ln = std::cmp::min(reversed.len(), s.len());
                    if ct == max_ln && reversed.len() < s.len() {
                        *s = reversed.clone();
                    }
                }

                dfs(root.left.clone(), s, curr);
                dfs(root.right.clone(), s, curr);
                curr.pop();
            }
        }

        dfs(root, &mut res, &mut vec![]);

        res.iter().map(|a| (b'a' + *a as u8) as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::treenode::NONE;

    use super::*;

    #[test]
    fn test_subarrays_with_k_distinct_1() {
        let root = TreeNode::build_tree_from_vec(vec![0, 1, 2, 3, 4, 3, 4]);

        assert_eq!(Solution::smallest_from_leaf(root), "dba");
    }

    #[test]
    fn test_subarrays_with_k_distinct_2() {
        let root = TreeNode::build_tree_from_vec(vec![25, 1, 3, 1, 3, 0, 2]);

        assert_eq!(Solution::smallest_from_leaf(root), "adz");
    }

    #[test]
    fn test_subarrays_with_k_distinct_3() {
        let root = TreeNode::build_tree_from_vec(vec![2, 2, 1, NONE, 1, 0, NONE, 0]);

        assert_eq!(Solution::smallest_from_leaf(root), "abc");
    }

    #[test]
    fn test_subarrays_with_k_distinct_4() {
        let root = TreeNode::build_tree_from_vec(vec![5, 25]);

        assert_eq!(Solution::smallest_from_leaf(root), "zf");
    }
}
