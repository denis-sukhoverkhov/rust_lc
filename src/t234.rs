// 234. Palindrome Linked List
// https://leetcode.com/problems/palindrome-linked-list/

use crate::helpers::listnode::ListNode;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let sentinel = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));

        let mut stack = vec![];
        {
            let (mut slow, mut fast) = (sentinel.as_ref(), sentinel.as_ref());

            // find mid of linked list
            while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
                slow = slow.unwrap().next.as_ref();
                fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
            }

            {
                let mut second_part = slow.unwrap().next.as_ref();
                while let Some(n) = second_part {
                    stack.push(n.val);
                    second_part = n.next.as_ref();
                }
            }
        }

        let stack = stack.iter().rev();
        let mut curr = sentinel.as_ref().unwrap().next.as_ref();
        for i in stack {
            let n = curr.unwrap();
            if n.val != *i {
                return false;
            }

            curr = n.next.as_ref();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_1() {
        let head = ListNode::build_list_from_vec(vec![1, 2, 2, 1]);

        assert!(Solution::is_palindrome(head));
    }

    #[test]
    fn test_is_palindrome_2() {
        let head = ListNode::build_list_from_vec(vec![1, 2]);

        assert!(!Solution::is_palindrome(head));
    }

    #[test]
    fn test_is_palindrome_3() {
        let head = ListNode::build_list_from_vec(vec![1, 2, 3, 4, 3, 2, 1]);

        assert!(Solution::is_palindrome(head));
    }

    #[test]
    fn test_is_palindrome_4() {
        let head = ListNode::build_list_from_vec(vec![1, 2, 3, 4, 4, 3, 2, 1]);

        assert!(Solution::is_palindrome(head));
    }
}
