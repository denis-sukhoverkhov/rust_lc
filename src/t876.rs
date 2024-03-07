// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/

use crate::helpers::listnode::ListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = head.as_ref();
        let mut p2 = head.as_ref()?.next.as_ref();

        while let (Some(slow), Some(fast)) = (p1, p2) {
            p1 = slow.next.as_ref();

            if let Some(n) = fast.next.as_ref() {
                p2 = n.next.as_ref();
            } else {
                break;
            }
        }

        Some(p1.unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_node_1() {
        let head = ListNode::build_list_from_vec(vec![1, 2, 3, 4, 5]);

        let expected = ListNode::build_list_from_vec(vec![3, 4, 5]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn test_middle_node_2() {
        let head = ListNode::build_list_from_vec(vec![1, 2, 3, 4, 5, 6]);

        let expected = ListNode::build_list_from_vec(vec![4, 5, 6]);
        assert_eq!(Solution::middle_node(head), expected);
    }
}
