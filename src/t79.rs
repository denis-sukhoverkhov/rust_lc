// 79. Word Search
// https://leetcode.com/problems/word-search/

use std::collections::HashSet;

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();

        let mut visited = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if Solution::backtrack(i as i32, j as i32, 0, &board, &word, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    fn backtrack(
        i: i32,
        j: i32,
        k: i32,
        b: &Vec<Vec<char>>,
        w: &Vec<char>,
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if k >= w.len() as i32 {
            return true;
        }

        if i < 0 || i >= b.len() as i32 || j < 0 || j >= b[0].len() as i32 {
            return false;
        }

        let i = i as usize;
        let j = j as usize;
        let k = k as usize;

        if b[i][j] != w[k] {
            return false;
        }

        if visited.contains(&(i, j)) {
            return false;
        }
        visited.insert((i, j));

        let res = Solution::backtrack(i as i32 - 1, j as i32, (k + 1) as i32, b, w, visited)
            || Solution::backtrack((i + 1) as i32, j as i32, (k + 1) as i32, b, w, visited)
            || Solution::backtrack(i as i32, j as i32 - 1, (k + 1) as i32, b, w, visited)
            || Solution::backtrack(i as i32, (j + 1) as i32, (k + 1) as i32, b, w, visited);

        visited.remove(&(i, j));

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings_1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        assert!(Solution::exist(board, word));
    }
}
