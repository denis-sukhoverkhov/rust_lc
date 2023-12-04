// 2264. Largest 3-Same-Digit Number in String
// https://leetcode.com/problems/largest-3-same-digit-number-in-string/description/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn largest_good_integer(num: String) -> String {
        let mut numbers: Vec<String> = Vec::new();

        for i in (0..10).rev() {
            numbers.push(i.to_string().repeat(3));
        }

        let mut resp = "".to_string();
        for n in numbers {
            if num.contains(&n) {
                resp = n;
                break;
            }
        }

        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ways_0_chairs_1() {
        let num = "6777133339".to_string();

        assert_eq!(Solution::largest_good_integer(num), "777".to_string());
    }

    #[test]
    fn test_number_of_ways_0_chairs_2() {
        let num = "2300019".to_string();

        assert_eq!(Solution::largest_good_integer(num), "000".to_string());
    }

    #[test]
    fn test_number_of_ways_0_chairs_3() {
        let num = "42352338".to_string();

        assert_eq!(Solution::largest_good_integer(num), "".to_string());
    }
}
