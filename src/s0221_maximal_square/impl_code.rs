pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols]; rows];
        let mut max_side = 0;
        for i in 0..rows {
            for j in 0..cols {
                dp[i][j] = (matrix[i][j] as u8 - b'0') as i32;
                if dp[i][j] == 1 {
                    max_side = 1;
                }
            }
        }
        for i in 1..rows {
            for j in 1..cols {
                if dp[i][j] == 1 {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                    if dp[i][j] > max_side {
                        max_side = dp[i][j];
                    }
                }
            }
        }
        max_side * max_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_square() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_square(matrix), 4);
    }
}
