// 232. Implement Queue using Stacks
// https://leetcode.com/problems/implement-queue-using-stacks

struct MyQueue {
    #[allow(dead_code)]
    front_stack: Vec<i32>,
    #[allow(dead_code)]
    back_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    #[allow(dead_code)]
    fn new() -> Self {
        MyQueue {
            front_stack: vec![],
            back_stack: vec![],
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.back_stack.push(x);
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        if self.front_stack.is_empty() {
            while let Some(x) = self.back_stack.pop() {
                self.front_stack.push(x)
            }
        }

        self.front_stack.pop().unwrap()
    }

    #[allow(dead_code)]
    fn peek(&mut self) -> i32 {
        if self.front_stack.is_empty() {
            while let Some(x) = self.back_stack.pop() {
                self.front_stack.push(x)
            }
        }

        *self.front_stack.last().unwrap()
    }

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.front_stack.is_empty() && self.back_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis_1() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);

        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);

        assert!(!obj.empty());
    }
}
