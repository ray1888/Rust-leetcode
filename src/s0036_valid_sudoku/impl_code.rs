pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_seen = vec![vec![false; 9]; 9];
        let mut col_seen = vec![vec![false; 9]; 9];
        let mut box_seen = vec![vec![false; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                let x = (board[i][j] as u8 - b'1') as usize;
                let k = (i / 3) * 3 + j / 3;
                if row_seen[i][x] || col_seen[j][x] || box_seen[k][x] {
                    return false;
                }
                row_seen[i][x] = true;
                col_seen[j][x] = true;
                box_seen[k][x] = true;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_sudoku_valid() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_valid_sudoku_invalid() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
