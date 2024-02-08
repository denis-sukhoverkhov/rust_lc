// 279. Perfect Squares
// https://leetcode.com/problems/perfect-squares

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_squares(n: i32) -> i32 {
        let mut squares = vec![];

        for i in 1..=(n / 2 + 1) {
            squares.push(i * i)
        }

        let mut d = 1;
        let mut q = vec![n];
        let mut nq = vec![];

        while !q.is_empty() {
            for node in q.iter() {
                for sq in squares.iter() {
                    if node == sq {
                        return d;
                    }

                    if node < sq {
                        break;
                    }

                    nq.push(node - sq)
                }
            }

            d += 1;
            q = nq;
            nq = vec![];
        }

        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares_1() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_num_squares_2() {
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn test_num_squares_3() {
        assert_eq!(Solution::num_squares(1), 1);
    }
}
