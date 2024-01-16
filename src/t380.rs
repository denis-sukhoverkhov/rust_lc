// 380. Insert Delete GetRandom O(1)
// https://leetcode.com/problems/insert-delete-getrandom-o1

use rand::seq::IteratorRandom;
use std::collections::HashSet;

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    #[allow(dead_code)]
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            return false;
        }

        self.set.insert(val);

        true
    }

    #[allow(dead_code)]
    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    #[allow(dead_code)]
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();

        *self.set.iter().choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = RandomizedSet::new();

        let t = HashSet::from([1, 2]);

        assert!(obj.insert(1));
        assert!(!obj.remove(2));
        assert!(obj.insert(2));

        let rand_val = obj.get_random();
        assert!(t.contains(&rand_val))
    }
}
