use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub const NONE: i32 = -10000;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build_tree_from_vec(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if v.is_empty() {
            return None;
        }

        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let first_node: Option<Rc<RefCell<TreeNode>>> =
            Some(Rc::new(RefCell::new(TreeNode::new(v[0]))));

        queue.push_back(first_node.clone());

        let mut idx = 1;
        let max_idx = v.len() - 1;
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap().unwrap();
            if idx <= max_idx && v[idx] != NONE {
                let left = Some(Rc::new(RefCell::new(TreeNode::new(v[idx]))));
                node.borrow_mut().left = left.clone();
                queue.push_back(left);
            }
            idx += 1;

            if idx <= max_idx && v[idx] != NONE {
                let right = Some(Rc::new(RefCell::new(TreeNode::new(v[idx]))));
                node.borrow_mut().right = right.clone();
                queue.push_back(right);
            }
            idx += 1;
        }

        first_node
    }

    pub fn tree_to_str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::new();
        }

        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);

        let mut result_numbers: Vec<i32> = vec![];
        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap().unwrap();
            result_numbers.push(curr.borrow().val);
            if curr.borrow().left.is_some() {
                queue.push_back(curr.borrow().left.clone());
            } else if curr.borrow().val != NONE {
                queue.push_back(Some(Rc::new(RefCell::new(TreeNode::new(NONE)))));
            }

            if curr.borrow().right.is_some() {
                queue.push_back(curr.borrow().right.clone());
            } else if curr.borrow().val != NONE {
                queue.push_back(Some(Rc::new(RefCell::new(TreeNode::new(NONE)))));
            }
        }

        while !result_numbers.is_empty() {
            if *result_numbers.last().unwrap() != NONE {
                break;
            }
            result_numbers.pop();
        }

        let result: String = result_numbers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree_from_vec_1() {
        let nodes = vec![];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(TreeNode::tree_to_str(root), "");
    }

    #[test]
    fn test_build_tree_from_vec_2() {
        let nodes = vec![1, 2, 3, 4];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(TreeNode::tree_to_str(root), "1,2,3,4");
    }

    #[test]
    fn test_build_tree_from_vec_3() {
        let nodes = vec![1, 2, 3, NONE, 4];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(TreeNode::tree_to_str(root), "1,2,3,-10000,4");
    }

    #[test]
    fn test_build_tree_from_vec_4() {
        let nodes = vec![1, 2, 3, NONE, NONE, 4];
        let root = TreeNode::build_tree_from_vec(nodes);
        assert_eq!(TreeNode::tree_to_str(root), "1,2,3,-10000,-10000,4");
    }
}
