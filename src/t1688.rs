// 1688. Count of Matches in Tournament
// https://leetcode.com/problems/count-of-matches-in-tournament/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn number_of_matches(n: i32) -> i32 {
        let mut current = n;

        let mut matches = 0;
        while current > 1 {
            if current % 2 == 0 {
                current /= 2;
                matches += current;
            } else {
                let tmp = (current - 1) / 2;
                matches += tmp;
                current = tmp + 1;
            }
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_matches_1() {
        assert_eq!(Solution::number_of_matches(7), 6);
    }

    #[test]
    fn test_number_of_matches_2() {
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
