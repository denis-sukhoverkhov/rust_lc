// 1669. Merge In Between Linked Lists
// https://leetcode.com/problems/merge-in-between-linked-lists/

use crate::helpers::listnode::ListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode {
            val: -1,
            next: list1,
        });
        let mut deleted = Box::new(ListNode {
            val: -1,
            next: None,
        });

        let mut current_node = sentinel.as_mut();
        let mut p = 0;
        while current_node.next.is_some() {
            if p == a {
                deleted.next = current_node.next.take();
                current_node.next = list2.take();
            }
            p += 1;
            current_node = current_node.next.as_mut().unwrap();
        }

        let mut deleted_end = deleted.next.as_mut().unwrap();

        for _ in a..b {
            deleted_end = deleted_end.next.as_mut().unwrap();
        }

        current_node.next = deleted_end.next.take();

        sentinel.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_in_between_1() {
        let list1 = ListNode::build_list_from_vec(vec![10, 1, 13, 6, 9, 5]);
        let (a, b) = (3, 4);
        let list2 = ListNode::build_list_from_vec(vec![1000000, 1000001, 1000002]);

        let expected = ListNode::build_list_from_vec(vec![10, 1, 13, 1000000, 1000001, 1000002, 5]);

        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }

    #[test]
    fn test_merge_in_between_2() {
        let list1 = ListNode::build_list_from_vec(vec![0, 1, 2, 3, 4, 5, 6]);
        let (a, b) = (2, 5);
        let list2 =
            ListNode::build_list_from_vec(vec![1000000, 1000001, 1000002, 1000003, 1000004]);

        let expected = ListNode::build_list_from_vec(vec![
            0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6,
        ]);

        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }
}
