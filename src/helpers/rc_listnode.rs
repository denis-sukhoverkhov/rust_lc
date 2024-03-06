use std::{cell::RefCell, rc::Rc};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RcListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<RcListNode>>>,
}

impl RcListNode {
    #[inline]
    fn new(val: i32) -> Self {
        RcListNode { next: None, val }
    }

    pub fn build_list_from_vec(v: Vec<i32>) -> Option<Rc<RefCell<RcListNode>>> {
        let head = Rc::new(RefCell::new(RcListNode::new(0)));

        let mut curr_head = head.clone();
        for n in v.iter() {
            let next = Rc::new(RefCell::new(RcListNode::new(*n)));
            curr_head.borrow_mut().next = Some(next.clone());
            curr_head = next;
        }

        let r = head.borrow();
        r.next.clone()
    }

    pub fn build_vec_from_list(mut h: Option<Rc<RefCell<RcListNode>>>) -> Vec<i32> {
        let mut res = vec![];
        while let Some(n) = h {
            res.push(n.borrow().val);
            h = n.borrow().next.clone();
        }

        res
    }

    pub fn make_loop(mut h: Option<Rc<RefCell<RcListNode>>>, pos: i32) {
        if pos == -1 {
            return;
        }

        if h.is_none() {
            return;
        }

        let mut last_node = h.clone().unwrap();
        let mut pos_node = h.clone().unwrap();

        let mut curr_iter = 0;
        while let Some(n) = h {
            last_node = n.clone();
            h = n.borrow().next.clone();

            if curr_iter == pos {
                pos_node = n.clone();
            }

            curr_iter += 1;
        }

        last_node.borrow_mut().next = Some(pos_node.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_list_fom_vec_1() {
        let nodes = vec![];
        let head = RcListNode::build_list_from_vec(nodes);
        assert_eq!(RcListNode::build_vec_from_list(head), []);
    }

    #[test]
    fn test_build_list_fom_vec_2() {
        let nodes = vec![1, 2, 3, 4, 5];
        let head = RcListNode::build_list_from_vec(nodes);
        assert_eq!(RcListNode::build_vec_from_list(head), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_make_loop_1() {
        let nodes = vec![1, 2, 3];
        let head = RcListNode::build_list_from_vec(nodes);
        RcListNode::make_loop(head.clone(), 1);

        let h = head
            .unwrap()
            .borrow()
            .next
            .clone()
            .unwrap()
            .borrow()
            .next
            .clone()
            .unwrap()
            .borrow()
            .next
            .clone()
            .unwrap()
            .borrow()
            .val;

        assert_eq!(h, 2);
    }
}
