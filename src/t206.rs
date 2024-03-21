// 206. Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/

use crate::helpers::listnode::ListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Box::new(ListNode {
            val: -1,
            next: None,
        });

        while let Some(mut n) = head {
            let tmp = new_head.next;
            head = n.next;
            n.next = tmp;
            new_head.next = Some(n);
        }

        new_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob_1() {
        let head = ListNode::build_list_from_vec(vec![1, 2, 3, 4, 5]);

        let expected = ListNode::build_list_from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(head), expected);
    }

    #[test]
    fn test_rob_2() {
        let head = ListNode::build_list_from_vec(vec![1, 2]);

        let expected = ListNode::build_list_from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list(head), expected);
    }

    #[test]
    fn test_rob_3() {
        let head = ListNode::build_list_from_vec(vec![]);

        let expected = ListNode::build_list_from_vec(vec![]);
        assert_eq!(Solution::reverse_list(head), expected);
    }
}
