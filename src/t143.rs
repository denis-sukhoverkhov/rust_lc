// 143. Reorder List
// https://leetcode.com/problems/reorder-list/

use std::collections::VecDeque;

use crate::helpers::listnode::ListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut deq = VecDeque::new();

        let mut curr_node = head.as_mut();
        while let Some(node) = curr_node {
            deq.push_back(node.val);
            curr_node = node.next.as_mut();
        }

        let mut take_from_front = true;
        let mut curr_node = head.as_mut();
        while let Some(node) = curr_node {
            let val = if take_from_front {
                deq.pop_front().unwrap()
            } else {
                deq.pop_back().unwrap()
            };

            take_from_front = !take_from_front;

            node.val = val;
            curr_node = node.next.as_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_list_1() {
        let mut head = ListNode::build_list_from_vec(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut head);

        let expected = ListNode::build_list_from_vec(vec![1, 4, 2, 3]);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_reorder_list_2() {
        let mut head = ListNode::build_list_from_vec(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut head);

        let expected = ListNode::build_list_from_vec(vec![1, 5, 2, 4, 3]);
        assert_eq!(head, expected);
    }
}
