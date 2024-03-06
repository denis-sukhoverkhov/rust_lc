// 141. Linked List Cycle
// https://leetcode.com/problems/linked-list-cycle

use std::{cell::RefCell, rc::Rc};

use crate::helpers::rc_listnode::RcListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn has_cycle(head: Option<Rc<RefCell<RcListNode>>>) -> bool {
        if head.is_none() {
            return false;
        }

        let mut sp = head.clone();
        let mut fp = head.clone().unwrap().borrow().next.clone();

        while let (Some(slow), Some(fast)) = (sp, fp) {
            let s_value = slow.borrow().val;
            let f_value = fast.borrow().val;

            if s_value == f_value {
                return true;
            }

            sp = slow.clone().borrow().next.clone();
            fp = fast.clone().borrow().next.clone();

            match fp {
                Some(ref f) => {
                    fp = f.clone().borrow().next.clone();
                }
                None => break,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle_1() {
        let head = RcListNode::build_list_from_vec(vec![3, 2, 0, -4]);
        RcListNode::make_loop(head.clone(), 1);

        assert!(Solution::has_cycle(head));
    }

    #[test]
    fn test_has_cycle_2() {
        let head = RcListNode::build_list_from_vec(vec![1, 2]);
        RcListNode::make_loop(head.clone(), 0);

        assert!(Solution::has_cycle(head));
    }

    #[test]
    fn test_has_cycle_3() {
        let head = RcListNode::build_list_from_vec(vec![1]);
        RcListNode::make_loop(head.clone(), -1);

        assert!(!Solution::has_cycle(head));
    }

    #[test]
    fn test_has_cycle_4() {
        let head = RcListNode::build_list_from_vec(vec![1]);
        RcListNode::make_loop(head.clone(), 0);

        assert!(Solution::has_cycle(head));
    }

    #[test]
    fn test_has_cycle_5() {
        let head = RcListNode::build_list_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        RcListNode::make_loop(head.clone(), 9);

        assert!(Solution::has_cycle(head));
    }
}
