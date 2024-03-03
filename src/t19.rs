// 19. Remove Nth Node From End of List
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

use crate::helpers::listnode::ListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut sentinel = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));

        let mut h2 = &(sentinel.clone());

        for _ in 0..n {
            h2 = &h2.as_ref()?.next;
        }

        let mut h1 = &mut sentinel;
        while h2.is_some() && h2.as_ref()?.next.is_some() {
            h2 = &h2.as_ref()?.next;
            h1 = &mut h1.as_mut()?.next;
        }

        h1.as_mut()?.next = h1.as_mut()?.next.as_mut()?.next.take();

        sentinel?.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::listnode::build_list_from_vec;

    #[test]
    fn test_remove_nth_from_end_1() {
        let head = build_list_from_vec(vec![1, 2, 3, 4, 5]);
        let n = 2;

        let expected = build_list_from_vec(vec![1, 2, 3, 5]);
        assert_eq!(Solution::remove_nth_from_end(head, n), expected);
    }

    #[test]
    fn test_remove_nth_from_end_2() {
        let head = build_list_from_vec(vec![1]);
        let n = 1;

        let expected = build_list_from_vec(vec![]);
        assert_eq!(Solution::remove_nth_from_end(head, n), expected);
    }

    #[test]
    fn test_remove_nth_from_end_3() {
        let head = build_list_from_vec(vec![1, 2]);
        let n = 1;

        let expected = build_list_from_vec(vec![1]);
        assert_eq!(Solution::remove_nth_from_end(head, n), expected);
    }

    #[test]
    fn test_remove_nth_from_end_4() {
        let head = build_list_from_vec(vec![1, 2, 3, 4, 5]);
        let n = 3;

        let expected = build_list_from_vec(vec![1, 2, 4, 5]);
        assert_eq!(Solution::remove_nth_from_end(head, n), expected);
    }
}
