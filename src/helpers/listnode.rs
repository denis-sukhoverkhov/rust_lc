// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn build_list_from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));

    let mut curr_head = head.as_mut();
    for n in v.iter() {
        curr_head.next = Some(Box::new(ListNode::new(*n)));
        curr_head = curr_head.next.as_mut().unwrap();
    }

    head.next
}

pub fn build_vec_from_list(mut h: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = vec![];
    while let Some(ref n) = h {
        res.push(n.val);
        h = n.next.clone();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_list_fom_vec_1() {
        let nodes = vec![];
        let head: Option<Box<ListNode>> = build_list_from_vec(nodes);
        assert_eq!(build_vec_from_list(head), []);
    }

    #[test]
    fn test_build_list_fom_vec_2() {
        let nodes = vec![1, 2, 3, 4, 5];
        let head: Option<Box<ListNode>> = build_list_from_vec(nodes);
        assert_eq!(build_vec_from_list(head), [1, 2, 3, 4, 5]);
    }
}
