// 2125. Number of Laser Beams in a Bank
// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut res = 0;

        let mut prev_sec_dev_ct = 0;
        for r in bank {
            let curr_sec_dev_ct = r.chars().filter(|c| *c == '1').count() as i32;
            if curr_sec_dev_ct == 0 {
                continue;
            }

            res += prev_sec_dev_ct * curr_sec_dev_ct;
            prev_sec_dev_ct = curr_sec_dev_ct;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_beams_1() {
        let bank = vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ];
        assert_eq!(Solution::number_of_beams(bank), 8);
    }

    #[test]
    fn test_number_of_beams_2() {
        let bank = vec!["000".to_string(), "111".to_string(), "000".to_string()];
        assert_eq!(Solution::number_of_beams(bank), 0);
    }
}
