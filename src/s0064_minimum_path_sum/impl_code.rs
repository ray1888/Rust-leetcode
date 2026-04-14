pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let length = grid.len();
        let width = grid[0].len();
        let mut dp = vec![vec![0; width]; length];
        dp[0][0] = grid[0][0];
        let mut l_sum = 0;
        for i in 0..length {
            l_sum += grid[i][0];
            dp[i][0] = l_sum;
        }
        let mut w_sum = 0;
        for j in 0..width {
            w_sum += grid[0][j];
            dp[0][j] = w_sum;
        }
        for i in 1..length {
            for j in 1..width {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j];
            }
        }
        dp[length - 1][width - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 1], vec![8, 4, 1], vec![-8, 2, 1]]),
            4
        );
    }
}
