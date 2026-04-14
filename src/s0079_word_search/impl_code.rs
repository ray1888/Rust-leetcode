pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() && word.is_empty() {
            return true;
        }
        if board.is_empty() {
            return false;
        }
        let row = board.len();
        let col = board[0].len();
        let mut visited = vec![vec![false; col]; row];
        let word_bytes = word.as_bytes();
        for i in 0..row {
            for j in 0..col {
                if Self::dfs(&board, &mut visited, i, j, word_bytes, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        word: &[u8],
        idx: usize,
    ) -> bool {
        if idx >= word.len() {
            return true;
        }
        let row = board.len();
        let col = board[0].len();
        if i >= row || j >= col || visited[i][j] || board[i][j] as u8 != word[idx] {
            return false;
        }
        visited[i][j] = true;
        let existed = (i + 1 < row && Self::dfs(board, visited, i + 1, j, word, idx + 1))
            || (j > 0 && Self::dfs(board, visited, i, j - 1, word, idx + 1))
            || (i > 0 && Self::dfs(board, visited, i - 1, j, word, idx + 1))
            || (j + 1 < col && Self::dfs(board, visited, i, j + 1, word, idx + 1));
        visited[i][j] = false;
        existed
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist(board.clone(), "ABCCED".to_string()));
        assert!(Solution::exist(board.clone(), "SEE".to_string()));
        assert!(!Solution::exist(board, "ABCB".to_string()));
    }
}
