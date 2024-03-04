// 948. Bag of Tokens
// https://leetcode.com/problems/bag-of-tokens/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        tokens.sort();

        let (mut l, mut r) = (0_usize, tokens.len() - 1);

        let (mut score, mut max_score) = (0, 0);
        while l <= r {
            if tokens[l] <= power {
                score += 1;
                max_score = max_score.max(score);
                power -= tokens[l];

                l += 1;
                continue;
            }

            if score > 0 {
                power += tokens[r];
                score -= 1;

                r -= 1;
                continue;
            } else {
                break;
            }
        }

        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_of_tokens_score_0() {
        let tokens = vec![];
        let power = 85;

        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 0);
    }

    #[test]
    fn test_bag_of_tokens_score_1() {
        let tokens = vec![100];
        let power = 50;

        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 0);
    }

    #[test]
    fn test_bag_of_tokens_score_2() {
        let tokens = vec![200, 100];
        let power = 150;

        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 1);
    }

    #[test]
    fn test_bag_of_tokens_score_3() {
        let tokens = vec![100, 200, 300, 400];
        let power = 200;

        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 2);
    }
}
