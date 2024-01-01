// 455. Assign Cookies
// https://leetcode.com/problems/assign-cookies

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut p1 = (g.len() - 1) as i32;
        let mut p2 = (s.len() - 1) as i32;

        let mut res = 0;
        loop {
            if p1 < 0 || p2 < 0 {
                break;
            }

            if s[p2 as usize] >= g[p1 as usize] {
                res += 1;
                p2 -= 1;
            }

            p1 -= 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children_1() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];

        assert_eq!(Solution::find_content_children(g, s), 1);
    }

    #[test]
    fn test_find_content_children_2() {
        let g = vec![1, 2];
        let s = vec![1, 2, 3];

        assert_eq!(Solution::find_content_children(g, s), 2);
    }
}
