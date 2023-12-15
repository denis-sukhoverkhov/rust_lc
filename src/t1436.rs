// 1436. Destination City
// https://leetcode.com/problems/destination-city/

use std::collections::HashMap;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map: HashMap<String, String> = HashMap::new();

        for path in paths {
            let (from, to) = (path[0].clone(), path[1].clone());
            map.insert(from, to);
        }

        let values = map.values();

        for v in values {
            if !map.contains_key(v) {
                return v.clone();
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_1() {
        let paths = vec![
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "B".to_string()],
            vec!["C".to_string(), "A".to_string()],
        ];

        assert_eq!(Solution::dest_city(paths), "A".to_string())
    }

    #[test]
    fn test_max_product_2() {
        let paths = vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()],
        ];

        assert_eq!(Solution::dest_city(paths), "Sao Paulo".to_string())
    }

    #[test]
    fn test_max_product_3() {
        let paths = vec![vec!["A".to_string(), "Z".to_string()]];

        assert_eq!(Solution::dest_city(paths), "Z".to_string())
    }
}
