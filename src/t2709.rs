// 2709. Greatest Common Divisor Traversal
// https://leetcode.com/problems/greatest-common-divisor-traversal/

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut factor_to_index = HashMap::new();
        let mut tree = HashMap::new();

        if nums.len() == 1 {
            return true;
        }

        for (i, n) in nums.iter().enumerate() {
            let mut f = 2;

            let mut curr_n = *n;
            while f * f <= curr_n {
                if n % f == 0 {
                    let idx = factor_to_index.entry(f).or_insert(i as i32);

                    let set_of_idx = tree.entry(*idx).or_insert(HashSet::new());
                    if *idx != i as i32 {
                        set_of_idx.insert(i as i32);

                        let reverse_idx = tree.entry(i as i32).or_insert(HashSet::new());
                        reverse_idx.insert(*idx);
                    }

                    while curr_n % f == 0 {
                        curr_n /= f;
                    }
                }
                f += 1;
            }

            if curr_n > 1 {
                let idx: &mut i32 = factor_to_index.entry(curr_n).or_insert(i as i32);

                let set_of_idx = tree.entry(*idx).or_insert(HashSet::new());
                if *idx != i as i32 {
                    set_of_idx.insert(i as i32);

                    let reverse_idx = tree.entry(i as i32).or_insert(HashSet::new());
                    reverse_idx.insert(*idx);
                }
            }
        }

        let len = nums.len();
        let mut visited: HashSet<i32> = HashSet::new();
        fn dfs(
            tree: &HashMap<i32, HashSet<i32>>,
            visited: &mut HashSet<i32>,
            idx: i32,
            len: usize,
        ) {
            if !tree.contains_key(&idx) {
                return;
            }

            if visited.contains(&idx) {
                return;
            }

            visited.insert(idx);

            if visited.len() == len {
                return;
            }

            let curr_set = tree.get(&idx).unwrap();
            for n in curr_set.iter() {
                dfs(tree, visited, *n, len);
            }
        }

        dfs(&tree, &mut visited, 0, len);

        visited.len() == len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_traverse_all_pairs_1() {
        let nums = vec![2, 3, 6];

        assert!(Solution::can_traverse_all_pairs(nums));
    }

    #[test]
    fn test_can_traverse_all_pairs_2() {
        let nums = vec![3, 9, 5];

        assert!(!Solution::can_traverse_all_pairs(nums));
    }

    #[test]
    fn test_can_traverse_all_pairs_3() {
        let nums = vec![4, 3, 12, 8];

        assert!(Solution::can_traverse_all_pairs(nums));
    }

    #[test]
    fn test_can_traverse_all_pairs_4() {
        let nums = vec![1];

        assert!(Solution::can_traverse_all_pairs(nums));
    }
}
