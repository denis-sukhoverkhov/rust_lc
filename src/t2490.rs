#[derive(Default)]
struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let first = sentence.chars().next();
        let last = sentence.chars().last();

        let res = match (first, last) {
            (Some(first), Some(last)) => first == last,
            _ => false,
        };

        if !res {
            return false;
        }

        let mut chars = sentence.chars().peekable();
        let mut prev: Option<char> = None;

        while let Some(c) = chars.next() {
            if c == ' ' {
                if let Some(prev) = prev {
                    if let Some(next) = chars.peek() {
                        if next != &prev {
                            return false;
                        }
                    }
                }
            }

            prev = Some(c);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_circular_sentence_1() {
        assert!(!Solution::is_circular_sentence("Hello World".to_string()));
    }

    #[test]
    fn test_is_circular_sentence_2() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ));
    }
}
