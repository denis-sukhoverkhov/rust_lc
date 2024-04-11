// 402. Remove K Digits
// https://leetcode.com/problems/remove-k-digits/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<char> = vec![];
        for c in num.chars() {
            while let Some(head) = stack.pop() {
                if head <= c || k == 0 {
                    stack.push(head);
                    break;
                }

                k -= 1;
            }

            stack.push(c);
        }

        let res: String = stack
            .iter()
            .take(stack.len() - k as usize)
            .skip_while(|&&x| x == '0')
            .collect();

        if res.is_empty() {
            return String::from("0");
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = "1432219".to_string();
        let k = 3;

        assert_eq!(Solution::remove_kdigits(nums, k), "1219");
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = "10200".to_string();
        let k = 1;

        assert_eq!(Solution::remove_kdigits(nums, k), "200");
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = "10".to_string();
        let k = 2;

        assert_eq!(Solution::remove_kdigits(nums, k), "0");
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = "1234567890".to_string();
        let k = 3;

        assert_eq!(Solution::remove_kdigits(nums, k), "1234560");
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = "9".to_string();
        let k = 1;

        assert_eq!(Solution::remove_kdigits(nums, k), "0");
    }

    #[test]
    fn test_find_duplicates_6() {
        let nums = "112".to_string();
        let k = 1;

        assert_eq!(Solution::remove_kdigits(nums, k), "11");
    }

    #[test]
    fn test_find_duplicates_7() {
        let nums = "10001".to_string();
        let k = 1;

        assert_eq!(Solution::remove_kdigits(nums, k), "1");
    }

    #[test]
    fn test_find_duplicates_8() {
        let nums = "10001".to_string();
        let k = 4;

        assert_eq!(Solution::remove_kdigits(nums, k), "0");
    }
}
