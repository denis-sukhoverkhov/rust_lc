// 1287. Element Appearing More Than 25% In Sorted Array
// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let elems = arr.len();
        let mut curr_elem: i32 = arr[0];
        let mut ct_curr = 0;
        for a in arr {
            if curr_elem != a {
                curr_elem = a;
                ct_curr = 0;
            }
            ct_curr += 1;
            let perc = ct_curr as f32 * 100.0 / elems as f32;

            if perc > 25_f32 {
                return curr_elem;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_special_integer_1() {
        let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];

        assert_eq!(Solution::find_special_integer(arr), 6);
    }

    #[test]
    fn test_find_special_integer_2() {
        let arr = vec![1, 1];

        assert_eq!(Solution::find_special_integer(arr), 1);
    }
}
