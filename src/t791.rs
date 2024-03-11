// 791. Custom Sort String
// https://leetcode.com/problems/custom-sort-string/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut m = HashMap::new();

        for c in s.chars() {
            let v = m.entry(c).or_insert(0);
            *v += 1;
        }

        let mut res = String::new();

        for c in order.chars() {
            if m.contains_key(&c) {
                let v = m.get(&c).unwrap();
                res.push_str(&c.to_string().repeat(*v));
                m.remove(&c);
            }
        }

        let mut keys: Vec<_> = m.keys().collect();
        keys.sort();
        for key in keys.iter() {
            let v = m.get(key).unwrap();
            res.push_str(&key.to_string().repeat(*v));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_sort_string_1() {
        let order = String::from("cba");
        let s = String::from("abcd");

        let expected = String::from("cbad");

        assert_eq!(Solution::custom_sort_string(order, s), expected);
    }

    #[test]
    fn test_custom_sort_string_2() {
        let order = String::from("bcafg");
        let s = String::from("abcd");

        let expected = String::from("bcad");

        assert_eq!(Solution::custom_sort_string(order, s), expected);
    }
}
